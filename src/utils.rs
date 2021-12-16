use anyhow::Result;
use std::os;
use std::path::{Path, PathBuf};
use chrono::prelude::*;


fn fallback_path() -> PathBuf {
    PathBuf::from(".")
}

fn create_cryptokeep_path(path: &mut PathBuf) -> Result<()> {
    path.push(".cryptokeep");
    std::fs::create_dir(path)?;
    Ok(())
}

pub fn get_home_dir_path() -> Result<PathBuf> {
    let home_path = std::env::home_dir();

    let mut path = match home_path {
        Some(p) => {
            if !p.exists() || !p.is_dir() {
                fallback_path()
            } else { p }
        }
        None => fallback_path()
    };

    create_cryptokeep_path(&mut path);
    Ok(path)
}

pub fn current_date_time() -> String {
    let local = Local::now();
    local.to_string()
}
