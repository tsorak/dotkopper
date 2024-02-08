use std::path::Path;
use std::process::exit;

pub fn cfg_not_found(cfg_path: &Path) {
    eprintln!(
        "Could not find a config at the specified path.\n{}",
        cfg_path.display()
    );
    exit(1);
}

pub fn home_var_not_found() {
    eprintln!("Could not find the current users HOME.");
    exit(1);
}
