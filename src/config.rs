use std::{fs, path::PathBuf};

pub fn open_config(path: &str) -> Vec<(String, String)> {
    let cfg = fs::read_to_string(path).unwrap();
    let lines = cfg.split("\n").collect::<Vec<&str>>();
    let cfg_entries: Vec<Option<(String, String)>> = lines
        .iter()
        .map(|line| {
            let words = line
                .split(" ")
                .map(|str| str.to_owned())
                .collect::<Vec<String>>();

            if words.len() == 2 {
                let (origin, target) = (words[0].to_owned(), words[1].to_owned());
                if valid_target(&target) {
                    Some((origin, target))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    cfg_entries.iter().cloned().filter_map(|e| e).collect()
}

fn valid_target(t: &String) -> bool {
    if t.starts_with("~/") || t.starts_with("/") {
        true
    } else {
        false
    }
}

pub fn get_cfg_parent(p: &PathBuf) -> Option<String> {
    let p = p.clone();

    let ends_with_cfg_name = p
        .file_name()
        .expect("Bad config path specified.")
        .to_str()
        .unwrap()
        == "dotkopper";

    if p.is_dir() {
        let p_contains_cfg = p.join("dotkopper").is_file();

        if p_contains_cfg {
            Some(p.to_str().unwrap().to_string())
        } else {
            None
        }
    } else if p.is_file() {
        if ends_with_cfg_name {
            Some(p.parent().unwrap().to_str().unwrap().to_string())
        } else {
            None
        }
    } else {
        None
    }
}

pub fn unwrap_cfg_entries(cfg: Vec<(Option<String>, Option<String>)>) -> Vec<(String, String)> {
    cfg.iter()
        .filter_map(|p| {
            let (o, t) = p.clone();

            if o.is_some() && t.is_some() {
                Some((o.unwrap(), t.unwrap()))
            } else {
                None
            }
        })
        .collect()
}

pub mod parsers {
    use std::{env, path::PathBuf};

    pub fn absolute_paths(
        origin_files_root: &String,
        cfg: &Vec<(String, String)>,
    ) -> Vec<(Option<String>, Option<String>)> {
        let home = env::var("HOME").expect("Could not find HOME of the current user.");
        let home = PathBuf::from(home);

        let origin_files_root = PathBuf::from(origin_files_root);

        cfg.iter()
            .map(|pair| {
                let (origin, target) = pair.clone();

                // the config should be placed in the root of a dotfiles git repo,
                // thus all origin paths should be specified in a relative form.
                //
                // ex: Say we have a dotfiles repo containing both a "dotkopper" and a
                // ".tmux.conf" file.
                // The "dotkopper" file would then contain one of the following entries:
                // ./.tmux.conf ~/.tmux.conf
                // .tmux.conf ~/.tmux.conf
                let origin = if origin.starts_with("/") {
                    None
                } else if valid_origin_start(&origin) {
                    let origin = omit_dot_slash(&origin);
                    Some(origin_files_root.join(origin).to_str().unwrap().to_string())
                } else {
                    None
                };

                let target = if target.starts_with("/") {
                    Some(target)
                } else if target.starts_with("~/") {
                    Some(absolute_home_path(&home, target))
                } else {
                    None
                };

                (origin, target)
            })
            .collect()
    }

    fn absolute_home_path(homedir: &PathBuf, p: String) -> String {
        let target = p.strip_prefix("~/").unwrap();

        homedir.join(target).to_str().unwrap().to_string()
    }

    fn valid_origin_start(s: &String) -> bool {
        let mut s = s.clone();
        s.remove(0).is_ascii()
    }

    fn omit_dot_slash(s: &String) -> String {
        s.strip_prefix("./").unwrap_or(s).to_string()
    }
}
