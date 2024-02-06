use std::fmt::Debug;

use crate::config::*;

pub(crate) struct DotConfig {
    pub path: String,
    pub entries: Vec<Dotfile>,
}

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
    pub fn new() -> DotConfig {
        let cfg_path = get_cfg_path();

        DotConfig {
            path: cfg_path.clone(),
            entries: open_config(&cfg_path),
        }
    }

    pub fn map_origins<F>(&mut self, f: F)
    where
        Self: Sized,
        F: Fn(&str) -> String,
    {
        self.entries.iter_mut().for_each(|df| {
            df.origin = f(&df.origin);
        });
    }

    fn map_targets<F>(&mut self, f: F)
    where
        Self: Sized,
        F: Fn(&str) -> String,
    {
        self.entries.iter_mut().for_each(|df| {
            df.target = f(&df.target);
        });
    }
}

impl Dotfile {
    pub fn new(origin: String, target: String) -> Dotfile {
        Dotfile { origin, target }
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
