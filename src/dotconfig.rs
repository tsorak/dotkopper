use std::{fmt::Debug, path::PathBuf};

mod fs_checks;
mod load_config;
mod path_parsers;
mod reporters;

use crate::utils::exit;

#[derive(Debug)]
pub(crate) struct DotConfig {
    pub path: PathBuf,
    entries: Vec<Dotfile>,
    home_dir: String,
}

#[derive(Clone)]
struct Dotfile {
    pub origin: Box<PathBuf>,
    pub target: Box<PathBuf>,
    target_status: Option<TargetStatus>,
}

impl Debug for Dotfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} -> {}",
            self.target_status_human_readable(),
            self.origin.display(),
            self.target.display()
        )
    }
}

#[derive(Clone)]
enum TargetStatus {
    Unlinked = 0,
    Occupied = 1,
    Linked = 2,
}

impl DotConfig {
    pub fn new() -> Self {
        let cfg_path = DotConfig::get_cfg_path().unwrap_or_else(|error| {
            eprintln!("{error}");
            std::process::exit(1);
        });

        let home_dir = match std::env::var("HOME") {
            Ok(s) => s,
            Err(_) => {
                exit::home_var_not_found();
                unreachable!();
            }
        };

        DotConfig {
            path: cfg_path,
            entries: vec![],
            home_dir,
        }
    }

    pub fn init(&mut self) -> &mut Self {
        self.load_config()
            .report_bad_origin_paths()
            .reject_invalid_origins()
            .absolute_origins()
            .absolute_targets()
            .append_origin_filename_to_target_dirs()
            .update_target_statuses()
            .report_target_statuses()
            .filter_valid_targets()
    }

    fn dbg(&mut self) -> &mut Self {
        dbg!(self)
    }

    fn report_bad_origin_paths(&mut self) -> &mut Self {
        self.entries
            .iter()
            .for_each(|df| df.report_bad_origin_path());
        self
    }

    fn reject_invalid_origins(&mut self) -> &mut Self {
        self.entries.retain(|df| df.is_valid_origin());
        self
    }

    fn absolute_origins(&mut self) -> &mut Self {
        let relative_path_stem = self.path.parent().unwrap();

        self.entries = self
            .entries
            .iter_mut()
            .map(|dotfile| dotfile.absolute_origin(relative_path_stem))
            .collect();
        self
    }

    fn absolute_targets(&mut self) -> &mut Self {
        self.entries = self
            .entries
            .iter_mut()
            .filter_map(|dotfile| dotfile.absolute_target(&self.home_dir))
            .collect();
        self
    }

    fn append_origin_filename_to_target_dirs(&mut self) -> &mut Self {
        self.entries = self
            .entries
            .iter_mut()
            .map(|dotfile| dotfile.target_with_origin_filename())
            .collect();
        self
    }

    fn update_target_statuses(&mut self) -> &mut Self {
        self.entries = self
            .entries
            .iter_mut()
            .map(|dotfile| dotfile.update_target_status())
            .collect();
        self
    }

    fn report_target_statuses(&mut self) -> &mut Self {
        self.entries
            .iter()
            .for_each(|dotfile| dotfile.report_target_status());
        self
    }

    fn filter_valid_targets(&mut self) -> &mut Self {
        self.entries
            .retain(|df| matches!(df.target_status, Some(TargetStatus::Unlinked)));
        self
    }
}

impl Dotfile {
    pub fn new(o: &str, t: &str) -> Dotfile {
        let origin: Box<PathBuf> = PathBuf::from(o).into();
        let target: Box<PathBuf> = PathBuf::from(t).into();

        Dotfile {
            origin,
            target,
            target_status: None,
        }
    }

    fn is_valid_origin(&self) -> bool {
        self.origin.starts_with("./")
    }

    fn target_status_human_readable(&self) -> &str {
        match self.target_status {
            Some(TargetStatus::Linked) => "LINKED",
            Some(TargetStatus::Occupied) => "OCCUPIED",
            Some(TargetStatus::Unlinked) => "UNLINKED",
            None => "UNSET",
        }
    }
}

impl From<(&'static str, &'static str)> for Dotfile {
    fn from(v: (&str, &str)) -> Self {
        let origin: Box<PathBuf> = PathBuf::from(v.0).into();
        let target: Box<PathBuf> = PathBuf::from(v.1).into();

        Dotfile {
            origin,
            target,
            target_status: None,
        }
    }
}

impl TryFrom<&str> for Dotfile {
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.splitn(2, ' ').collect::<Vec<&str>>()[..] {
            [o, t, ..] => Ok(Self::new(o, t)),
            _ => Err(()),
        }
    }
    type Error = ();
}
