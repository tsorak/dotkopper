use std::{fmt::Debug, path::Path};

mod parser;
use parser::*;

use crate::{config::*, utils::exit};

#[derive(Debug)]
pub(crate) struct DotConfig {
    pub path: Box<Path>,
    pub entries: Vec<Dotfile>,
    home_dir: String,
}

#[derive(Clone)]
pub(crate) struct Dotfile {
    pub origin: Box<Path>,
    pub target: Box<Path>,
}

impl Debug for Dotfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}",
            self.origin.to_string_lossy(),
            self.target.to_string_lossy()
        )
    }
}

impl DotConfig {
    pub fn new() -> Self {
        let cfg_path = get_cfg_path();

        let home_dir = match std::env::var("HOME") {
            Ok(s) => s,
            Err(_) => {
                exit::home_var_not_found();
                unreachable!();
            }
        };

        DotConfig {
            path: cfg_path,
            entries: vec![],
            home_dir,
        }
    }

    pub fn init(&mut self) -> &mut Self {
        self.load_config().absolute_origins().absolute_targets()
    }

    fn absolute_origins(&mut self) -> &mut Self {
        self.entries = self
            .entries
            .iter_mut()
            .filter_map(|dotfile| dotfile.absolute_origin())
            .collect();
        self
    }

    fn absolute_targets(&mut self) -> &mut Self {
        self.entries = self
            .entries
            .iter_mut()
            .filter_map(|dotfile| dotfile.absolute_target(&self.home_dir))
            .collect();
        self
    }
}

impl Dotfile {
    pub fn new(o: &str, t: &str) -> Dotfile {
        let origin: Box<Path> = Path::new(o).into();
        let target: Box<Path> = Path::new(t).into();

        Dotfile { origin, target }
    }

    fn absolute_origin(&mut self) -> Option<Dotfile> {
        let p = &mut self.origin;
        match p.canonicalize() {
            Ok(absolute_path) => {
                self.origin = absolute_path.into();
                Some(self.clone())
            }
            Err(_) => None,
        }
    }

    fn absolute_target(&mut self, home_dir: &str) -> Option<Dotfile> {
        let p = &self.target;
        match parse_target(p, home_dir) {
            Some(absolute_path) => {
                self.target = absolute_path.into();
                Some(self.clone())
            }
            None => None,
        }
    }
}

impl From<(&'static str, &'static str)> for Dotfile {
    fn from(v: (&str, &str)) -> Self {
        let origin: Box<Path> = Path::new(v.0).into();
        let target: Box<Path> = Path::new(v.1).into();

        Dotfile { origin, target }
    }
}

impl TryFrom<&str> for Dotfile {
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.splitn(2, ' ').collect::<Vec<&str>>()[..] {
            [o, t, ..] => Ok(Self::new(o, t)),
            _ => Err(()),
        }
    }
    type Error = ();
}
