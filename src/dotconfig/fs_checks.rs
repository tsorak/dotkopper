use super::Dotfile;

impl Dotfile {
    pub(super) fn target_exists(&self) -> bool {
        self.target.exists()
    }
}
