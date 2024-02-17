use std::io::{self, Write};

mod dotconfig;
use dotconfig::{DotConfig, LinkError};

fn main() {
    let cfg = DotConfig::new();
    println!("Using config '{}'...", &cfg.path.display());
    let cfg = cfg.init();

    if !cfg.has_linkable_dotfiles() {
        println!("\nNo dotfiles to link.");
        std::process::exit(1);
    }

    wait_for_enter();

    match cfg.create_symlinks() {
        Ok(_) => println!("Links created successfully!"),
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

    cfg.update_target_statuses().report_statuses();
}

fn wait_for_enter() {
    let mut input = String::new();
    println!();
    print!("Press Enter to link files...");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!();
}
