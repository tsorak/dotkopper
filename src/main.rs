#![feature(fs_try_exists)]
#![feature(absolute_path)]

mod config;

pub mod dotconfig;

use dotconfig::*;

mod utils;

mod path_validator;

fn main() {
    let mut cfg = DotConfig::new();
    cfg.init();
    println!("Using config '{}'...", &cfg.path);

    dbg!(&cfg);
}
