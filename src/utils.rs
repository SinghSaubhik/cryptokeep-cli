use std::fmt::Display;
use anyhow::Result;
use std::path::{PathBuf};
use chrono::prelude::*;
use console::{style};

fn create_path(path: &PathBuf) -> Result<()> {
    // If path does not exists, create a new path
    if !std::path::Path::exists(path) {
        std::fs::create_dir(path)?;
    }
    Ok(())
}

fn fallback_path() -> PathBuf {
    PathBuf::from(".")
}

fn create_cryptokeep_path(path: &mut PathBuf) -> Result<()> {
    path.push(".cryptokeep");
    create_path(&path)
}


#[allow(deprecated)]
pub fn home_dir() -> PathBuf {
    let home_path = std::env::home_dir();

    let path = match home_path {
        Some(p) => {
            if !p.exists() || !p.is_dir() {
                fallback_path()
            } else { p }
        }
        None => fallback_path()
    };

    path
}

pub fn get_component_path(filename: &str) -> Result<PathBuf> {
    let mut path = get_app_dir_path().unwrap();
    path.push(filename);
    create_path(&path);

    Ok(path)
}

#[allow(deprecated)]
pub fn get_app_dir_path() -> Result<PathBuf> {
    let mut path = home_dir();

    create_cryptokeep_path(&mut path).expect("Unable to create path");
    Ok(path)
}

pub fn current_date_time() -> String {
    let local = Local::now();
    local.to_string()
}

pub enum Level {
    ERROR,
    SUCCESS,
    INFO,
    BRIGHTBOLD,
}

pub fn write_color<T: Display>(msg: T, level: Level) {
    let st;

    match level {
        Level::INFO => { st = style(msg).bright().cyan(); }
        Level::SUCCESS => { st = style(msg).bright().green(); }
        Level::ERROR => { st = style(msg).red().bright(); }
        Level::BRIGHTBOLD => { st = style(msg).bold().bright() }
    }

    println!("  {}  ", st);
}
