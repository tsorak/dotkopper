use std::{
    env,
    path::{Path, PathBuf},
};

pub fn absolute_paths(
    origin_files_root: &String,
    cfg: &[(String, String)],
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
            let origin = if origin.starts_with('/') {
                None
            } else if valid_origin_start(&origin) {
                let origin = omit_dot_slash(&origin);
                Some(origin_files_root.join(origin).to_str().unwrap().to_string())
            } else {
                None
            };

            let target = if target.starts_with('/') {
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

fn absolute_home_path(homedir: &Path, p: String) -> String {
    let target = p.strip_prefix("~/").unwrap();

    homedir.join(target).to_str().unwrap().to_string()
}

fn valid_origin_start(s: &str) -> bool {
    let mut s = s.to_string();
    s.remove(0).is_ascii()
}

fn omit_dot_slash(s: &String) -> String {
    s.strip_prefix("./").unwrap_or(s).to_string()
}
