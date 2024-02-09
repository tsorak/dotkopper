use super::Dotfile;

impl Dotfile {
    pub(super) fn target_exists(&self) -> bool {
        self.target.exists()
    }

    pub(super) fn absolute_origin(&mut self) -> Option<Dotfile> {
        let p = &mut self.origin;
        match p.canonicalize() {
            Ok(absolute_path) => {
                self.origin = absolute_path.into();
                Some(self.clone())
            }
            Err(_) => None,
        }
    }
}
