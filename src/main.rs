#![feature(fs_try_exists)]
#![feature(absolute_path)]

mod config;
use config::config::*;

mod safety_checks;
use safety_checks::safety_checks::*;

use std::{env, path};

fn main() -> () {
    let args = env::args().collect::<Vec<String>>();

    let cfg_arg = match args.get(1) {
        Some(s) => s.to_owned(),
        None => ".".to_owned(),
    };
    let cfg_path = path::absolute(cfg_arg).unwrap();
    let cfg_path = append_cfg_to_path(cfg_path.to_str().unwrap());
    println!("Using config '{}'...", cfg_path);

    let cfg = open_config(&cfg_path);
    let cfg = reject_missing_origin_files(&cfg);
    let cfg = absolutify_home_paths(&cfg);
    dbg!(cfg);
}
