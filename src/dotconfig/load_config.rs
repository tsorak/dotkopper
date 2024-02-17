use std::{env, fs, path::PathBuf};

use super::DotConfig;

impl DotConfig {
    pub(super) fn load_config(mut self) -> Self {
        self.entries = fs::read_to_string(&self.path)
            .unwrap()
            .lines()
            .filter_map(|line| {
                if line.starts_with('#') {
                    None
                } else {
                    match line.try_into() {
                        Ok(dotfile) => Some(dotfile),
                        Err(_) => None,
                    }
                }
            })
            .collect();

        self
    }

    pub(super) fn get_cfg_path() -> Result<PathBuf, String> {
        let cfg_arg = env::args().nth(1);
        let cwd = env::current_dir().unwrap_or_else(|_| {
            eprintln!("Could not get path of cwd");
            std::process::exit(1);
        });

        match cfg_arg {
            Some(user_input) => {
                let specified_path = cwd.join(user_input);
                find_config_at(&specified_path)
            }
            None => find_config_at(&cwd),
        }
    }
}

fn find_config_at(path: &PathBuf) -> Result<PathBuf, String> {
    let file_name = path.file_name().unwrap();

    match path.metadata() {
        Ok(m) if (m.is_file() && file_name == "dotkopper") => Ok(path.to_owned()),
        Ok(m) if (m.is_dir() && path.join("dotkopper").is_file()) => Ok(path.join("dotkopper")),
        Ok(m) if (m.is_dir()) => Err(format!("No config found in '{}'", path.display())),
        Ok(_) => Err(format!("Bad config path specified '{}'", path.display())),
        Err(_) => Err(format!("Could not read '{}'", path.display())),
    }
}
