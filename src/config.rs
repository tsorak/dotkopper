use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::dotconfig::*;
use crate::std_ext::path::ContainsFile;
use crate::utils::exit;

impl DotConfig {
    pub fn load_config(&mut self) -> &mut Self {
        self.entries = fs::read_to_string(&self.path)
            .unwrap_or_else(|_| {
                exit::cfg_not_found(self.path.clone());
                unreachable!();
            })
            .lines()
            .filter_map(|line| {
                let df: Option<Dotfile> = match line.try_into() {
                    Ok(df) => Some(df),
                    Err(_) => None,
                };
                df
            })
            .collect();

        self
    }
}

pub fn get_cfg_path() -> PathBuf {
    let cfg_arg = match env::args().nth(1) {
        Some(s) => s,
        None => ".".to_owned(),
    };

    let p = Path::new(&cfg_arg).canonicalize().unwrap();
    if p.contains_file("dotkopper") {
        p.join("dotkopper")
    } else {
        p
    }
}
