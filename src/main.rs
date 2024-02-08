mod config;

mod dotconfig;
use dotconfig::*;

mod utils;

mod std_ext;

fn main() {
    let mut cfg = DotConfig::new();
    cfg.init();
    println!("Using config '{}'...", &cfg.path);

    dbg!(&cfg);
}
