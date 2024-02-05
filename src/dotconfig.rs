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

// impl std::fmt::Display for Vec<Dotfile> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "\n");
//         for ele in self.iter() {
//             write!(f, "{}", ele);
//         }
//         write!(f, "\n")
//     }
// }

impl Debug for Dotfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(
        //     f,
        //     "{{ \n origin: {},\n target: {},\n}}",
        //     self.origin, self.target
        // )
        write!(f, "{} -> {}", self.origin, self.target)
    }
}
