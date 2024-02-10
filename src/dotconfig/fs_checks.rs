use std::path::Path;

use super::Dotfile;

impl Dotfile {
    pub(super) fn target_exists(&self) -> bool {
        self.target.exists()
    }

    pub(super) fn absolute_origin(&mut self, relative_path_stem: &Path) -> Self {
        if let ['.', '/', origin_path @ ..] =
            &self.origin.to_str().unwrap().chars().collect::<Vec<char>>()[..]
        {
            let o: String = origin_path.iter().collect();
            self.origin = Box::new(relative_path_stem.join(o));
        };

        self.clone()
    }
}
