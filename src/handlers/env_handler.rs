use std::collections::HashMap;
use std::fs;
use std::error::Error;

// TODO: Add filter as well
pub fn parse_and_return(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let variables = fs::read_to_string(path)?.lines().map(|line| {
        let mut split = line.splitn(2, '=');
        let key = split.next().unwrap_or("").to_string();
        let value = split.next().unwrap_or("").to_string();
        (key, value)
    }).collect::<HashMap<String, String>>();

    return Ok(variables);
}