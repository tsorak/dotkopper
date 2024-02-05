pub mod parsers;

use std::{fs, path::PathBuf};

pub fn open_config(path: &str) -> Vec<(String, String)> {
    let cfg = fs::read_to_string(path).unwrap();
    let lines = cfg.split("\n").collect::<Vec<&str>>();
    let cfg_entries: Vec<Option<(String, String)>> = lines
        .iter()
        .map(|line| {
            let words = line
                .split(" ")
                .map(|str| str.to_owned())
                .collect::<Vec<String>>();

            if words.len() == 2 {
                let (origin, target) = (words[0].to_owned(), words[1].to_owned());
                if valid_target(&target) {
                    Some((origin, target))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    cfg_entries.iter().cloned().filter_map(|e| e).collect()
}

fn valid_target(t: &String) -> bool {
    if t.starts_with("~/") || t.starts_with("/") {
        true
    } else {
        false
    }
}

pub fn get_cfg_parent(p: &PathBuf) -> Option<String> {
    let p = p.clone();

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
        .filter_map(|p| {
            let (o, t) = p.clone();

            if o.is_some() && t.is_some() {
                Some((o.unwrap(), t.unwrap()))
            } else {
                None
            }
        })
        .collect()
}

pub fn unwrap_cfg_pairs(cfg: Vec<Option<(String, String)>>) -> Vec<(String, String)> {
    cfg.iter()
        .map(|p| p.clone())
        .filter_map(|p| if p.is_some() { Some(p.unwrap()) } else { None })
        .collect()
}
