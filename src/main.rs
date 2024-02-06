#![feature(fs_try_exists)]
#![feature(absolute_path)]

mod config;
use config::*;

mod dotconfig;
use dotconfig::*;

mod utils;
use utils::exit;

mod path_validator;

fn main() {
    let cfg_path = get_cfg_path();
    println!("Using config '{}'...", cfg_path);

    let cfg = DotConfig {
        path: cfg_path.clone(),
        entries: open_config(&cfg_path),
    };

    dbg!(cfg);
}
