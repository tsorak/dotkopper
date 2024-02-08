pub(crate) mod path {
    use std::path::Path;

    pub trait ContainsFile {
        fn contains_file(&self, f: &str) -> bool;
    }

    impl ContainsFile for Path {
        fn contains_file(&self, f: &str) -> bool {
            if self.is_dir() {
                self.join(f).is_file()
            } else {
                false
            }
        }
    }
}
