use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

mod fs_checks;
mod load_config;
mod path_parsers;

use crate::utils::exit;

#[derive(Debug)]
pub(crate) struct DotConfig {
    pub path: Box<Path>,
    entries: Vec<Dotfile>,
    home_dir: String,
}

#[derive(Clone)]
struct Dotfile {
    pub origin: Box<PathBuf>,
    pub target: Box<PathBuf>,
}

impl Debug for Dotfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}",
            self.origin.to_string_lossy(),
            self.target.to_string_lossy()
        )
    }
}

impl DotConfig {
    pub fn new() -> Self {
        let cfg_path = DotConfig::get_cfg_path();

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
            .absolute_origins()
            .absolute_targets()
            .append_origin_filename_to_target_dirs()
            .filter_nonexistent_targets()
    }

    fn absolute_origins(&mut self) -> &mut Self {
        self.entries = self
            .entries
            .iter_mut()
            .filter_map(|dotfile| dotfile.absolute_origin())
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

    fn filter_nonexistent_targets(&mut self) -> &mut Self {
        self.entries = self
            .entries
            .iter()
            .filter_map(|dotfile| match dotfile.target_exists() {
                false => Some(dotfile.clone()),
                true => None,
            })
            .collect();
        self
    }
}

impl Dotfile {
    pub fn new(o: &str, t: &str) -> Dotfile {
        let origin: Box<PathBuf> = PathBuf::from(o).into();
        let target: Box<PathBuf> = PathBuf::from(t).into();

        Dotfile { origin, target }
    }
}

impl From<(&'static str, &'static str)> for Dotfile {
    fn from(v: (&str, &str)) -> Self {
        let origin: Box<PathBuf> = PathBuf::from(v.0).into();
        let target: Box<PathBuf> = PathBuf::from(v.1).into();

        Dotfile { origin, target }
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
