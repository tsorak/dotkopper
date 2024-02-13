use std::io::{self, Write};

mod dotconfig;
use dotconfig::DotConfig;

fn main() {
    let mut cfg = DotConfig::new();
    println!("Using config '{}'...", &cfg.path.display());
    cfg.init();

    wait_for_enter();

    let link_result = cfg.create_symlinks();
    match link_result {
        Ok(_) => println!("YIPPIE!"),
        Err(error) => {
            println!("EEEEEEEEEEEEEEEEERGH");
            dbg!(error);
        }
    };
}

fn wait_for_enter() {
    let mut input = String::new();
    print!("Press Enter to continue...");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}
