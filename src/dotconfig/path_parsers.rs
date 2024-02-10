use std::path::{Path, PathBuf};

use super::Dotfile;

fn parse_target(p: &Path, home_dir: &str) -> Option<PathBuf> {
    let s = p.to_str()?;
    match s.chars().collect::<Vec<_>>()[..] {
        ['~', '/', ..] => {
            let mut t = s.get(1..).unwrap().to_owned();
            t.insert_str(0, home_dir);
            Some(t.into())
        }
        ['/', ..] => Some(p.into()),
        _ => None,
    }
}

impl Dotfile {
    pub(super) fn absolute_target(&mut self, home_dir: &str) -> Option<Dotfile> {
        let p = &self.target;
        match parse_target(p, home_dir) {
            Some(absolute_path) => {
                self.target = absolute_path.into();
                Some(self.clone())
            }
            None => None,
        }
    }

    pub(super) fn target_with_origin_filename(&mut self) -> Dotfile {
        let Dotfile {
            origin: o,
            target: t,
        } = self;

        if t.to_str().unwrap().ends_with('/') {
            let origin_filename = o.file_name().unwrap();
            self.target = Box::new(self.target.join(origin_filename));
        }

        self.clone()
    }
}
