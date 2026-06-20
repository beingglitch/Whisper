use std::{collections::HashMap, fs, path::Path};
use std::io::Write;

pub fn read_env(path: &Path) -> Option<HashMap<String, String>> {
    let env_string = fs::read_to_string(path).ok()?;

    let env_content: HashMap<String, String> = env_string
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split_once("="))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    Some(env_content)
}

pub fn sync_env(path: &Path, env_variables: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(mut env_content) = read_env(path) {
        for (key, value) in &env_variables {
            env_content.insert(key.clone(), value.clone());
        }

        write_env(path, env_content)?;
    }

    Ok(())
}

fn write_env(path: &Path, env_variables: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = fs::File::create(path)?;

    let env_new_content = env_variables.iter().map(|(key, value)| format!("{key}={value}")).collect::<Vec<String>>().join("\n");

    f.write_all(env_new_content.as_bytes())?;

    Ok(())
}