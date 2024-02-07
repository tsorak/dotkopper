mod config;

mod dotconfig;
use dotconfig::*;

mod utils;

fn main() {
    let mut cfg = DotConfig::new();
    cfg.init();
    println!("Using config '{}'...", &cfg.path);

    dbg!(&cfg);
}
