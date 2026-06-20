use std::{collections::HashMap, fs, path::Path};

pub async fn read_env(path: &Path) -> Option<HashMap<String, String>> {
    let env_string = fs::read_to_string(path).ok()?;

    let env_content: HashMap<String, String> = env_string
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split_once("="))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    Some(env_content)
}

pub async fn sync_env(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}