pub mod safety_checks {
    use std::{env, fs, path};

    pub fn reject_missing_origin_files(cfg: &Vec<(String, String)>) -> Vec<(String, String)> {
        let absolutified_cfg: Vec<(Option<String>, String)> = cfg
            .clone()
            .iter()
            .map(|pair| {
                let origin = pair.0.clone();
                let origin = path::absolute(origin).unwrap().to_str().unwrap().to_owned();
                let exists = fs::try_exists(&origin).unwrap();

                if !exists {
                    println!("{} does not exist, skipping...", origin);
                    (None, pair.1.clone())
                } else {
                    (Some(origin), pair.1.clone())
                }
            })
            .collect();

        let valid_origins_cfg: Vec<(String, String)> = absolutified_cfg
            .iter()
            .filter(|pair| pair.0.is_some())
            .map(|p| {
                let (origin, target) = p.to_owned();
                (origin.unwrap(), target)
            })
            .collect();

        valid_origins_cfg
    }

    pub fn absolutify_home_paths(cfg: &Vec<(String, String)>) -> Vec<(String, String)> {
        let home = env::var("HOME").unwrap();
        let home = path::Path::new(&home);

        cfg.iter()
            .map(|p| {
                let (origin, target) = p.to_owned();
                if target.starts_with("~/") {
                    let target = target.strip_prefix("~/").unwrap();
                    let target = home.join(target).to_str().unwrap().to_owned();

                    (origin, target)
                } else {
                    (origin, target)
                }
            })
            .collect()
    }
}

// fn get_dir_contents(dir: &str) -> Vec<(String, (bool, bool))> {
//     fs::read_dir(dir)
//         .unwrap()
//         .map(|e| {
//             let entry_path = match e.unwrap().path().to_str() {
//                 Some(str) => str.to_owned(),
//                 None => "".to_owned(),
//             };
//
//             let (is_dir, is_file) = (|| {
//                 let m = fs::metadata(entry_path.clone()).unwrap();
//                 (m.is_dir(), m.is_file())
//             })();
//
//             (entry_path.clone(), (is_dir, is_file))
//         })
//         .collect()
// }
