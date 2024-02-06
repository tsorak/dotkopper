use std::fs::{metadata, read_link};
use std::path::PathBuf;

pub fn omit_missing_origin_paths(cfg: Vec<(String, String)>) -> Vec<(String, String)> {
    cfg.iter()
        .filter_map(|p| {
            let o: &str = p.0.as_ref();

            let o = PathBuf::from(o);

            if o.is_file() {
                Some(p.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn omit_non_symlink_target_paths(cfg: Vec<(String, String)>) -> Vec<(String, String)> {
    cfg.iter()
        .filter_map(|p| {
            let t_str: &str = p.1.as_ref();
            let o: &str = p.0.as_ref();

            let t = PathBuf::from(t_str);

            if t_str.to_string().ends_with('/') {
                eprintln!("BAD TARGET '{}' points to a directory, skipping.", t_str);
                None
            } else {
                match metadata(&t) {
                    Ok(m) => {
                        if m.is_file() {
                            match read_link(&t) {
                                Ok(targets_origin) => {
                                    let t = t.to_string_lossy().to_string();
                                    let targets_origin =
                                        targets_origin.to_string_lossy().to_string();

                                    let correctly_linked = t == targets_origin;
                                    if correctly_linked {
                                        println!("OK: '{}' -> '{}'", o, t);
                                    } else {
                                        eprintln!(
                                            "Target '{}' is already linked to '{}', skipping.",
                                            t, targets_origin
                                        );
                                    }
                                }
                                Err(_e) => {
                                    eprintln!(
                                        "There is already a file at '{}', skipping.",
                                        t.to_string_lossy()
                                    );
                                }
                            }
                        } else if m.is_dir() {
                            eprintln!("Target '{}' is a directory, skipping.", t.to_string_lossy());
                        } else {
                            dbg!(m);
                            panic!("Unknown target encountered... symlink?");
                        };
                        None
                    }
                    Err(_e) => Some(p.clone()),
                }
            }
        })
        .collect()
}
