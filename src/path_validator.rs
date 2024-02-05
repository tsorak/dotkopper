use std::path::PathBuf;

pub fn omit_missing_origin_paths(cfg: Vec<(String, String)>) -> Vec<(String, String)> {
    cfg.iter()
        .map(|p| {
            let o: &str = p.0.as_ref();

            let o = PathBuf::from(o);

            if o.is_file() {
                Some(p.clone())
            } else {
                None
            }
        })
        .filter_map(|p| if p.is_some() { Some(p.unwrap()) } else { None })
        .collect()
}
