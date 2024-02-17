use std::{fs, path::Path};

use super::{Dotfile, TargetStatus};

impl Dotfile {
    pub(super) fn update_target_status(&mut self) {
        let Dotfile { origin, target, .. } = self;

        let status = match target.try_exists() {
            Err(_read_error) => TargetStatus::Occupied,
            Ok(false) => TargetStatus::Unlinked,
            Ok(true) if is_link(target) && is_linked_to(target, origin) => TargetStatus::Linked,
            Ok(true) if is_link(target) => TargetStatus::Occupied,
            Ok(true) => TargetStatus::Occupied,
        };
        self.target_status = Some(status);
    }

    pub(super) fn origin_exists(&self) -> bool {
        self.origin.exists()
    }
}

fn is_link(p: &Path) -> bool {
    fs::read_link(p).is_ok()
}

fn is_linked_to(target: &Path, origin: &Path) -> bool {
    let targets_origin = target.read_link().unwrap();

    targets_origin == origin
}
