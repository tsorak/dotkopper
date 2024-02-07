use std::{env, fs, path::Path};

use crate::dotconfig::*;
use crate::utils::exit;

pub fn get_cfg_path() -> String {
    let args = env::args().collect::<Vec<String>>();

    let cfg_arg = match args.get(1) {
        Some(s) => s.to_owned(),
        None => ".".to_owned(),
    };
    let maybe_cfg_or_cfg_dir = Path::new(&cfg_arg).canonicalize().unwrap();

    if maybe_cfg_or_cfg_dir.is_file() {
        maybe_cfg_or_cfg_dir.to_string_lossy().into()
    } else {
        let cfg_path_in_dir = maybe_cfg_or_cfg_dir.join("dotkopper");
        if cfg_path_in_dir.is_file() {
            cfg_path_in_dir.to_string_lossy().into()
        } else {
            exit::cfg_not_found(maybe_cfg_or_cfg_dir.to_string_lossy().into());
            unreachable!()
        }
    }
}

pub fn open_config(path: &str) -> Vec<Dotfile> {
    let cfg = fs::read_to_string(path).unwrap();
    let lines = cfg.split('\n').collect::<Vec<&str>>();
    lines
        .iter()
        .filter_map(|line| {
            let words = line
                .split(' ')
                .map(|str| str.to_owned())
                .collect::<Vec<String>>();

            if words.len() == 2 {
                let (origin, target) = (words[0].to_owned(), words[1].to_owned());
                if valid_target(&target) {
                    Some(Dotfile::new(origin, target))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn valid_target(t: &str) -> bool {
    t.starts_with("~/") || t.starts_with('/')
}
