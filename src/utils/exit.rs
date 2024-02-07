use std::env;
use std::process::exit;

pub fn cfg_not_found(cfg_arg: String) {
    let attempted_path = if cfg_arg.starts_with('/') {
        cfg_arg
    } else {
        env::current_dir().unwrap().to_string_lossy().to_string()
    };

    eprintln!(
        "Could not find a config at the specified path.\n{}",
        attempted_path
    );
    exit(1);
}

pub fn home_var_not_found() {
    eprintln!("Could not find the current users HOME.");
    exit(1);
}
