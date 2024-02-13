use std::io::{self, Write};

mod dotconfig;
use dotconfig::{DotConfig, LinkError};

fn main() {
    let mut cfg = DotConfig::new();
    println!("Using config '{}'...", &cfg.path.display());
    cfg.init();

    wait_for_enter();

    let link_result = cfg.create_symlinks();
    match link_result {
        Ok(_) => println!("YIPPIE!"),
        Err(LinkError(summary, errors)) => {
            println!("{}", summary);

            if !errors.is_empty() {
                errors
                    .iter()
                    .for_each(|err| eprintln!("{}", err.format_error()))
            }

            std::process::exit(1);
        }
    };
}

fn wait_for_enter() {
    let mut input = String::new();
    println!();
    print!("Press Enter to continue...");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!();
}
