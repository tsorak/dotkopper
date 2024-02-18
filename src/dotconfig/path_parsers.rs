use std::path::{Path, PathBuf};

use super::Dotfile;

fn make_target_absolute(p: &Path, home_dir: &Path) -> Option<PathBuf> {
    let s = p.to_str()?;
    match s.chars().collect::<Vec<_>>()[..] {
        ['~', '/', ..] => {
            let mut t = s.get(1..).unwrap().to_owned();
            t.insert_str(
                0,
                home_dir
                    .to_str()
                    .expect("Path to home contains illegal characters"),
            );
            Some(t.into())
        }
        ['/', ..] => Some(p.into()),
        _ => None,
    }
}

impl Dotfile {
    pub(super) fn absolute_target(&mut self, home_dir: &Path) -> Result<(), ()> {
        let t = &self.target;
        match make_target_absolute(t, home_dir) {
            Some(absolute_path) => {
                self.target = absolute_path;
                Ok(())
            }
            None => Err(()),
        }
    }

    pub(super) fn target_with_origin_filename(&mut self) {
        let Dotfile {
            origin: o,
            target: t,
            ..
        } = self;

        if t.to_str().unwrap().ends_with('/') {
            let origin_filename = o.file_name().unwrap();
            self.target = t.join(origin_filename);
        };
    }

    pub(super) fn absolute_origin(&mut self, relative_path_stem: &Path) {
        if let ['.', '/', origin_path @ ..] =
            &self.origin.to_str().unwrap().chars().collect::<Vec<char>>()[..]
        {
            let o: String = origin_path.iter().collect();
            self.origin = relative_path_stem.join(o);
        };
    }

    pub(super) fn is_valid_origin(&self) -> bool {
        self.origin.starts_with("./")
    }
}
