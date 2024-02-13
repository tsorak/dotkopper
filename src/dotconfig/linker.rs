use super::errors::DotfileLinkError;
use super::Dotfile;
use std::os;

impl Dotfile {
    pub(super) fn ensure_target_filetree_exists(&self) -> Option<DotfileLinkError> {
        let target_parent = self
            .target
            .parent()
            .expect("verify parent exists before calling me");

        match std::fs::create_dir_all(target_parent) {
            Err(e) => Some(DotfileLinkError::new(e, self.clone())),
            Ok(_) => None,
        }
    }

    #[cfg(target_os = "linux")]
    pub(super) fn create_symlink(&self) -> Result<(), DotfileLinkError> {
        let result = os::unix::fs::symlink(&*self.origin, &*self.target);
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(DotfileLinkError::new(e, self.clone())),
        }
    }
}
