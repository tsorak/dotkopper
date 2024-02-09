use std::path::{Path, PathBuf};

pub(super) fn parse_target(p: &Path, home_dir: &str) -> Option<PathBuf> {
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
