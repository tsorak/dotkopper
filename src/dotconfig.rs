use std::{fmt::Debug, path::Path};

mod parser;
use parser::*;

use crate::{config::*, utils::exit};

pub(crate) struct DotConfig {
    pub path: String,
    pub entries: Vec<Dotfile>,
    home_dir: String,
}

#[derive(Clone)]
pub(crate) struct Dotfile {
    pub origin: String,
    pub target: String,
}

impl Debug for DotConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n  path: {},\n  entries: {:#?},\n}}",
            self.path, self.entries
        )
    }
}

impl Debug for Dotfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", &self.origin, &self.target)
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
            path: cfg_path.clone(),
            entries: open_config(&cfg_path),
            home_dir,
        }
    }

    pub fn init(&mut self) -> &mut Self {
        self.absolute_origins().absolute_targets()
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

pub(crate) trait ParseDotfileRelation {
    fn parse_dotfile_relation(&self) -> Option<Dotfile>;
}

impl ParseDotfileRelation for (String, String) {
    fn parse_dotfile_relation(&self) -> Option<Dotfile> {
        let (origin, target) = self;

        Some(Dotfile {
            origin: origin.to_string(),
            target: target.to_string(),
        })
    }
}

impl ParseDotfileRelation for String {
    fn parse_dotfile_relation(&self) -> Option<Dotfile> {
        let words: Vec<&str> = self.split(' ').collect();
        match words[..] {
            [origin, target] => Some(Dotfile {
                origin: origin.to_string(),
                target: target.to_string(),
            }),
            _ => None,
        }
    }
}
