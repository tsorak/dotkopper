use super::{Dotfile, TargetStatus};

impl Dotfile {
    pub(super) fn report_bad_origin_path(&self) {
        let Self { origin: o, .. } = self;

        if !o.starts_with("./") {
            eprintln!("❌ BAD ORIGIN PATH '{}'", o.display());
        };
    }

    pub(super) fn report_target_status(&self) {
        let Dotfile {
            origin: o,
            target: t,
            target_status,
        } = self;

        let dbg_status = self.target_status_human_readable();

        let message = match *target_status {
            Some(TargetStatus::Unlinked) => {
                format!("Linking '{}' to '{}'", o.display(), t.display())
            }
            Some(TargetStatus::Occupied) => {
                format!("ERROR: A file exists at target path '{}'", t.display())
            }
            Some(TargetStatus::Linked) => {
                format!("Link exists '{}' -> '{}'", o.display(), t.display())
            }
            None => "ERROR: target_status not set".to_string(),
        };

        println!("[{}] {}", dbg_status, message);
    }
}
