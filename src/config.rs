pub mod config {
    use std::fs;

    pub fn open_config(path: &str) -> Vec<(String, String)> {
        let cfg = fs::read_to_string(path).unwrap();
        let lines = cfg.split("\n").collect::<Vec<&str>>();
        let cfg_entries: Vec<Option<(String, String)>> = lines
            .iter()
            .map(|line| {
                let words = line
                    .split(" ")
                    .map(|str| str.to_owned())
                    .collect::<Vec<String>>();

                if words.len() == 2 {
                    let (origin, target) = (words[0].to_owned(), words[1].to_owned());
                    if valid_target(&target) {
                        Some((origin, target))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        cfg_entries.iter().cloned().filter_map(|e| e).collect()
    }

    pub fn append_cfg_to_path(p: &str) -> String {
        if p.ends_with("/") {
            format!("{}{}", p, "dotkopper")
        } else if p.ends_with("dotkopper") {
            let maybe_cfg = p.to_owned();
            if fs::metadata(&maybe_cfg).unwrap().is_file() {
                maybe_cfg
            } else {
                format!("{}{}", maybe_cfg, "/dotkopper")
            }
        } else {
            format!("{}{}", p, "/dotkopper")
        }
    }

    fn valid_target(t: &String) -> bool {
        if t.starts_with("~/") || t.starts_with("/") {
            true
        } else {
            false
        }
    }
}
