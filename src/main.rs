mod dotconfig;
mod utils;

use dotconfig::DotConfig;

fn main() {
    let mut cfg = DotConfig::new();
    cfg.init();
    println!("Using config '{}'...", &cfg.path.display());

    dbg!(&cfg);
}
