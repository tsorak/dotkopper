use std::process::exit;

pub(crate) fn home_var_not_found() {
    eprintln!("Could not find the current users HOME.");
    exit(1);
}
