#![feature(fs_try_exists)]

mod config;
use config::config::*;

use std::{env, fs};

fn main() -> () {
    // let paths_in_dir = get_dir_contents(".");
    // dbg!(paths_in_dir);
    let cwdjunk = env::current_dir().unwrap();
    let cwd = cwdjunk.to_str().to_owned().ok_or("").unwrap();
    let cfg_path = append_cfg_to_path(cwd);
    println!("Using config '{}'...", cfg_path);

    let cfg = open_config(&cfg_path);
    dbg!(cfg);
}

fn get_dir_contents(dir: &str) -> Vec<(String, (bool, bool))> {
    fs::read_dir(dir)
        .unwrap()
        .map(|e| {
            let entry_path = match e.unwrap().path().to_str() {
                Some(str) => str.to_owned(),
                None => "".to_owned(),
            };

            let (is_dir, is_file) = (|| {
                let m = fs::metadata(entry_path.clone()).unwrap();
                (m.is_dir(), m.is_file())
            })();

            (entry_path.clone(), (is_dir, is_file))
        })
        .collect()
}
