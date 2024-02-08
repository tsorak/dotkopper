use std::{fmt::Debug, path::Path};

mod parser;
use parser::*;

use crate::{config::*, utils::exit};

#[derive(Debug)]
pub(crate) struct DotConfig {
    pub path: String,
    pub entries: Vec<Dotfile>,
    home_dir: String,
}

#[derive(Clone, Debug)]
pub(crate) struct Dotfile {
    pub origin: String,
    pub target: String,
}

impl DotConfig {
    pub fn new() -> Self {
        let cfg_path = get_cfg_path().to_string_lossy().into();

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
    pub fn new(origin: String, target: String) -> Dotfile {
        Dotfile { origin, target }
    }

    fn absolute_origin(&mut self) -> Option<Dotfile> {
        let p = Path::new(&self.origin);
        match p.canonicalize() {
            Ok(p) => {
                let mut v = self.clone();
                v.origin = p.to_string_lossy().into();
                Some(v)
            }
            Err(_) => None,
        }
    }

    fn absolute_target(&mut self, home_dir: &str) -> Option<Dotfile> {
        let p = Path::new(&self.target);
        match parse_target(p, home_dir) {
            Some(p) => {
                let mut v = self.clone();
                v.target = p.to_string_lossy().into();
                Some(v)
            }
            _ => None,
        }
    }
}

impl From<(&'static str, &'static str)> for Dotfile {
    fn from(v: (&str, &str)) -> Self {
        Self {
            origin: v.0.to_string(),
            target: v.1.to_string(),
        }
    }
}

impl TryFrom<&str> for Dotfile {
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.splitn(2, ' ').collect::<Vec<&str>>()[..] {
            [o, t, ..] => Ok(Self::new(o.to_string(), t.to_string())),
            _ => Err(()),
        }
    }
    type Error = ();
}
