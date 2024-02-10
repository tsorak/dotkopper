use std::{env, fs, path::PathBuf};

use super::{DotConfig, Dotfile};

impl DotConfig {
    pub(super) fn load_config(&mut self) -> &mut Self {
        self.entries = fs::read_to_string(&self.path)
            .unwrap()
            .lines()
            .filter_map(|line| {
                let df: Option<Dotfile> = match line.try_into() {
                    Ok(df) => Some(df),
                    Err(_) => None,
                };
                df
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
                dbg!(&user_input);
                let specified_path = cwd.join(&user_input);

                find_config_at(&specified_path)
            }
            None => find_config_at(&cwd),
        }
    }
}

fn find_config_at(path: &PathBuf) -> Result<PathBuf, String> {
    match path.metadata() {
        Ok(m) => {
            if m.is_dir() {
                let maybe_cfg = path.join("dotkopper");
                if maybe_cfg.is_file() {
                    let cfg = maybe_cfg;
                    Ok(cfg)
                } else {
                    Err(format!(
                        "Could not find a config in the directory '{}'",
                        path.display()
                    ))
                }
            } else if m.is_file() && path.file_name().unwrap() == "dotkopper" {
                Ok(path.to_owned())
            } else {
                Err(format!("Bad config path specified '{}'", path.display()))
            }
        }
        Err(_) => Err(format!(
            "Could not get information about '{}'",
            path.display()
        )),
    }
}
