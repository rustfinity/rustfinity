use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize)]
pub enum Difficulty {
    BEGINNER,
    EASY,
    MEDIUM,
    HARD,
    ADVANCED,
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
pub enum Track {
    RUST_BASICS,
    CONTROL_FLOW,
    DSA,
    RUST_STD,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CargoToml {
    pub package: Package,
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct Challenge {
    pub id: u32,
    pub title: String,
    pub slug: String,
    pub short_description: String,
    pub language: String,
    pub difficulty: Difficulty,
    pub track: Track,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn challenges_json() -> Result<Vec<Challenge>, std::io::Error> {
    let challenges_str = fs::read_to_string("challenges.json")?;
    let challenges = serde_json::from_str::<Vec<Challenge>>(&challenges_str)?;
    Ok(challenges)
}

pub fn challenges_dir_list() -> Result<Vec<PathBuf>, std::io::Error> {
    let mut entries = fs::read_dir("../challenges")?;
    let mut dirs = vec![];
    let ignored_dirs = ["src/", "target/"];

    while let Some(entry) = entries.next() {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let abs_path = path.canonicalize().unwrap();

            if ignored_dirs.iter().any(|&dir| abs_path.ends_with(dir)) {
                continue;
            }

            dirs.push(abs_path);
        }
    }

    Ok(dirs)
}

pub fn get_max_id() -> u32 {
    let challenges = challenges_json().unwrap();

    let mut max = 0;
    for challenge in challenges.iter() {
        if challenge.id > max {
            max = challenge.id;
        }
    }

    max
}
