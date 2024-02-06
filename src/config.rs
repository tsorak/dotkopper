pub mod parsers;

use std::{
    env, fs,
    path::{self, Path, PathBuf},
};

use crate::dotconfig::*;
use crate::exit;

pub fn get_cfg_path() -> String {
    let args = env::args().collect::<Vec<String>>();

    let cfg_arg = match args.get(1) {
        Some(s) => s.to_owned(),
        None => ".".to_owned(),
    };
    let maybe_cfg_or_cfg_dir = path::absolute(cfg_arg.clone()).unwrap();
    let (_cfg_parent_path, cfg_path) = {
        let parent_path = get_cfg_parent(&maybe_cfg_or_cfg_dir).unwrap_or_else(|| {
            exit::cfg_not_found(cfg_arg);
            unreachable!();
        });

        let cfg_path = PathBuf::from(&parent_path)
            .join("dotkopper")
            .to_string_lossy()
            .to_string();

        (parent_path, cfg_path)
    };

    cfg_path
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
                    Some(Dotfile { origin, target })
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

pub fn get_cfg_parent(p: &Path) -> Option<String> {
    let p = PathBuf::from(p);

    let ends_with_cfg_name = p
        .file_name()
        .expect("Bad config path specified.")
        .to_str()
        .unwrap()
        == "dotkopper";

    if p.is_dir() {
        let p_contains_cfg = p.join("dotkopper").is_file();

        if p_contains_cfg {
            Some(p.to_str().unwrap().to_string())
        } else {
            None
        }
    } else if p.is_file() {
        if ends_with_cfg_name {
            Some(p.parent().unwrap().to_str().unwrap().to_string())
        } else {
            None
        }
    } else {
        None
    }
}

pub fn unwrap_cfg_entries(cfg: Vec<(Option<String>, Option<String>)>) -> Vec<(String, String)> {
    cfg.iter()
        .filter_map(|p| match p.clone() {
            (Some(o), Some(t)) => Some((o, t)),
            _ => None,
        })
        .collect()
}

pub fn unwrap_cfg_pairs(cfg: Vec<Option<(String, String)>>) -> Vec<(String, String)> {
    cfg.iter().filter_map(|dotfile| dotfile.clone()).collect()
}
