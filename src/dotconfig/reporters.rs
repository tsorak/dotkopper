use super::Dotfile;

impl Dotfile {
    pub(super) fn report_bad_origin_path(&self) {
        let Self { origin: o, .. } = self;

        if !o.starts_with("./") {
            eprintln!("❌ BAD ORIGIN PATH '{}'", o.display());
        };
    }
}
