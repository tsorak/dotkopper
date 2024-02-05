use std::fmt::Debug;

pub(crate) struct DotConfig {
    pub path: String,
    pub entries: Vec<Dotfile>,
}

#[derive(Clone)]
pub(crate) struct Dotfile {
    pub origin: String,
    pub target: String,
}

impl Debug for DotConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n  path: {},\n  entries: {:#?},\n}}",
            self.path, self.entries
        )
    }
}

impl Debug for Dotfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.origin, self.target)
    }
}
