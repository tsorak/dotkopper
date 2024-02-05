#![feature(fs_try_exists)]
#![feature(absolute_path)]

mod config;
use config::parsers::*;
use config::*;

use std::{
    env,
    path::{self, PathBuf},
};

mod utils;
use utils::exit;

mod path_validator;
use path_validator::*;

// fn main() -> () {
//     let p = PathBuf::from("/somedir/target_is_dir_path/");
//
//     dbg!(p.to_string_lossy().to_string().ends_with("/"));
// }

fn main() -> () {
    let args = env::args().collect::<Vec<String>>();

    let cfg_arg = match args.get(1) {
        Some(s) => s.to_owned(),
        None => ".".to_owned(),
    };
    let maybe_cfg_or_cfg_dir = path::absolute(cfg_arg.clone()).unwrap();
    let (cfg_parent_path, cfg_path) = (|| {
        let parent_path = get_cfg_parent(&maybe_cfg_or_cfg_dir).unwrap_or_else(|| {
            exit::cfg_not_found(cfg_arg);
            unreachable!();
        });

        let cfg_path = PathBuf::from(&parent_path)
            .join("dotkopper")
            .to_string_lossy()
            .to_string();

        (parent_path, cfg_path)
    })();
    println!("Using config '{}'...", cfg_path);

    let cfg = open_config(&cfg_path);
    dbg!(&cfg);
    let cfg = absolute_paths(&cfg_parent_path, &cfg);
    let cfg = unwrap_cfg_entries(cfg);
    dbg!(&cfg);
    let cfg = omit_missing_origin_paths(cfg);
    dbg!(&cfg);
    let cfg = omit_non_symlink_target_paths(cfg);
    dbg!(&cfg);
}
