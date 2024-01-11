use std::{env, path::{Path, PathBuf}};

pub fn get_home_dir() -> PathBuf{
    match env::var("HOME") {
        Ok(home_dir) => Path::new(&home_dir).into(),
        Err(e) => Path::new("").into()
    }
}