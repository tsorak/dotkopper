use std::{
    path::Path,
    sync::Arc,
    thread::{self, JoinHandle},
};

use super::{DotConfig, Dotfile, TargetStatus};

impl DotConfig {
    pub fn init_multithreaded(self) -> Self {
        let mut conf = self.load_config();
        let dotfiles = conf.entries;

        let (home_dir_ref, origin_stem_ref) = {
            let home_dir = conf.home_dir.clone();
            let origin_stem = conf.path.parent().unwrap().to_owned();

            (Arc::new(home_dir), Arc::new(origin_stem))
        };

        let handles = dotfiles
            .into_iter()
            .map(|dotfile| {
                let stem = Arc::clone(&origin_stem_ref);
                let home = Arc::clone(&home_dir_ref);
                thread::spawn(move || parse_dotfile(dotfile, &stem, &home))
            })
            .collect::<Vec<JoinHandle<Option<Dotfile>>>>();

        let dotfiles = handles
            .into_iter()
            .filter_map(|h| match h.join() {
                Ok(Some(dotfile)) => Some(dotfile),
                Ok(None) => {
                    println!("Thread found its dotfile invalid");
                    None
                }
                Err(_) => {
                    eprintln!("Thread paniced while parsing dotfile");
                    None
                }
            })
            .collect();

        conf.entries = dotfiles;
        conf
    }
}

fn parse_dotfile(mut df: Dotfile, origin_stem: &Path, home_dir: &Path) -> Option<Dotfile> {
    if !df.is_valid_origin() {
        df.report_bad_origin_path();
        return None;
    }

    df.absolute_origin(origin_stem);

    if !df.origin_exists() {
        println!("Origin does not exist '{}'", df.origin.display());
        return None;
    }

    //

    if df.absolute_target(home_dir).is_err() {
        return None;
    }

    df.target_with_origin_filename();

    df.update_target_status();
    df.report_status();
    if matches!(df.target_status, Some(TargetStatus::Unlinked)) {
        Some(df)
    } else {
        None
    }
}
