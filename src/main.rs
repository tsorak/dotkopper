#![feature(fs_try_exists)]
#![feature(absolute_path)]

mod config;

pub mod dotconfig;

use dotconfig::*;

mod utils;

mod path_validator;

fn main() {
    let mut cfg = DotConfig::new();
    println!("Using config '{}'...", &cfg.path);

    dbg!(&cfg);
    dbg!(&cfg);

    cfg.map_origins(|_o| "hej".to_string());

    // if !cfg.entries.is_empty() {
    //     cfg.entries[0].origin.set(String::from("LUL!"));
    // };

    dbg!(&cfg);
}
