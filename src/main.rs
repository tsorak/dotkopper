mod dotconfig;
use dotconfig::DotConfig;

fn main() {
    let mut cfg = DotConfig::new();
    println!("Using config '{}'...", &cfg.path.display());
    cfg.init();
}
