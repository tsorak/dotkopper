use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

pub mod errors;
mod fs_checks;
mod linker;
mod load_config;
mod path_parsers;
mod reporters;

pub use self::errors::*;

#[derive(Debug)]
pub struct DotConfig {
    pub path: PathBuf,
    entries: Vec<Dotfile>,
    home_dir: String,
}

#[derive(Clone)]
struct Dotfile {
    pub origin: PathBuf,
    pub target: PathBuf,
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
                eprintln!("Could not find the current users HOME.");
                std::process::exit(1);
            }
        };

        DotConfig {
            path: cfg_path,
            entries: vec![],
            home_dir,
        }
    }

    pub fn init(self) -> Self {
        self.load_config()
            .report_bad_origin_paths()
            .reject_invalid_path_origins()
            .absolute_origins()
            .report_nonexistent_origins()
            .reject_nonexistent_origins()
            .absolute_targets()
            .append_origin_filename_to_target_dirs()
            .update_target_statuses()
            .report_statuses()
            .retain_unlinked_targets()
    }

    fn report_bad_origin_paths(self) -> Self {
        self.entries
            .iter()
            .for_each(|df| df.report_bad_origin_path());
        self
    }

    fn reject_invalid_path_origins(mut self) -> Self {
        self.entries.retain(|df| df.is_valid_origin());
        self
    }

    fn absolute_origins(mut self) -> Self {
        let relative_path_stem = self.path.parent().unwrap();

        self.entries.iter_mut().for_each(|dotfile| {
            dotfile.absolute_origin(relative_path_stem);
        });
        self
    }

    fn report_nonexistent_origins(self) -> Self {
        let errors = self
            .entries
            .iter()
            .filter_map(|df| {
                if !df.origin_exists() {
                    println!("Origin does not exist '{}'", df.origin.display());
                    Some(())
                } else {
                    None
                }
            })
            .collect::<Vec<()>>();

        if !errors.is_empty() {
            println!();
            println!("Origins should be specified relatively to the dotkopper config file.");
            println!("A sibling to the dotkopper config is written as './my_config_file.cfg'");
        };

        self
    }

    fn reject_nonexistent_origins(mut self) -> Self {
        self.entries.retain(|df| df.origin_exists());
        self
    }

    fn absolute_targets(mut self) -> Self {
        self.entries = self
            .entries
            .into_iter()
            .filter_map(
                |mut dotfile| match dotfile.absolute_target(&self.home_dir) {
                    Ok(_) => Some(dotfile),
                    Err(_) => None,
                },
            )
            .collect();

        self
    }

    fn append_origin_filename_to_target_dirs(mut self) -> Self {
        self.entries.iter_mut().for_each(|dotfile| {
            dotfile.target_with_origin_filename();
        });
        self
    }

    pub fn update_target_statuses(mut self) -> Self {
        self.entries.iter_mut().for_each(|dotfile| {
            dotfile.update_target_status();
        });
        self
    }

    pub fn report_statuses(self) -> Self {
        println!();
        self.entries
            .iter()
            .for_each(|dotfile| dotfile.report_status());
        self
    }

    fn retain_unlinked_targets(mut self) -> Self {
        self.entries
            .retain(|df| matches!(df.target_status, Some(TargetStatus::Unlinked)));
        self
    }

    #[cfg(target_os = "linux")]
    pub fn create_symlinks(&self) -> Result<(), LinkError> {
        if self.entries.is_empty() {
            return Err(LinkError::new("No dotfiles to link."));
        };

        let errors: Vec<DotfileLinkError> = self
            .entries
            .iter()
            .filter_map(|dotfile| {
                let t = &dotfile.target;
                if t.parent()
                    .is_some_and(|p| p.ne(Path::new("/")) && !p.exists())
                {
                    if let Some(error) = dotfile.ensure_target_filetree_exists() {
                        return Some(error);
                    };
                }

                match dotfile.create_symlink() {
                    Err(e) => Some(e),
                    Ok(_) => None,
                }
            })
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(LinkError("Error while linking dotfiles.", errors))
        }
    }

    pub fn has_linkable_dotfiles(&self) -> bool {
        if self.entries.is_empty() {
            return false;
        }

        let unlinked = self
            .entries
            .iter()
            .filter_map(|df| match df.target_status {
                Some(TargetStatus::Unlinked) => Some(()),
                _ => None,
            })
            .collect::<Vec<()>>();

        !unlinked.is_empty()
    }
}

impl Dotfile {
    pub fn new(o: &str, t: &str) -> Dotfile {
        Dotfile {
            origin: o.into(),
            target: t.into(),
            target_status: None,
        }
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
    fn from((o, t): (&str, &str)) -> Self {
        Dotfile {
            origin: o.into(),
            target: t.into(),
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
