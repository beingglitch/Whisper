use std::fs::File;
use std::io::prelude::*;
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

const PEER_PATH: &str = ".whisper/peers";
const ME_PATH: &str = ".whisper/me";
const ENV_PATH: &str = ".env";

#[derive(Debug, Serialize, Deserialize)]
struct SavedPeer {
    peer_id: String,
    peer_name: String
}

// ************************* Local Handlers *************************
pub fn init() {
    std::fs::create_dir(".whisper").unwrap();

    // Generating self peer id
    let mut file_me = File::create(ME_PATH).expect("Unable to Initialize whisper");

    let my_peer_id = Uuid::new_v4().to_string();

    file_me.write_all(my_peer_id.as_bytes()).unwrap();

    // Initializing peers file
    let mut file_peers = File::create(PEER_PATH).expect("Unable to Initialize whisper/peers");

    let saved_peers: Vec<SavedPeer> = Vec::new();
    file_peers.write_all(serde_json::to_string(&saved_peers).unwrap().as_bytes()).expect("Unable to Initialize whisper");

    // Writing .whisper to .gitignore file
    let mut file_gitignore = File::options().append(true).create(true).open(".gitignore").expect("Unable to Add into .gitignore, Add manually");

    file_gitignore.write_all(b"\n.whisper\n").unwrap();

    println!("Whisper Initialized.");
}

pub fn add_peer(peer_id: String) {

    let mut saved_peers = fetch_peer().unwrap();

    if let Some(_peer) = saved_peers.iter().find(|peer| peer.peer_id == peer_id) {
        println!("Peer: {} already exists", peer_id);
        return;
    }

    println!("Adding peer with ID: {}", peer_id);

    saved_peers.push( SavedPeer {peer_id: peer_id, peer_name: "Jane Doe".to_string()});

    let mut file = File::create(PEER_PATH).unwrap();
    file.write_all(serde_json::to_string(&saved_peers).unwrap().as_bytes()).expect("Not able to Add new peer!");

    println!("New Peer Added successfully!");
}

pub fn remove_peer(peer_id: String) {
    let mut saved_peers = fetch_peer().unwrap();

    if let Some(index) = saved_peers.iter().position(|peer| peer.peer_id == peer_id) {
        println!("Removing peer with ID: {}", peer_id);
        saved_peers.remove(index);

        let mut file = File::create(PEER_PATH).unwrap();
        file.write_all(serde_json::to_string(&saved_peers).unwrap().as_bytes()).expect("Not able to Add new peer!");

        println!("New Peer Added successfully!");
    } else {
        println!("Peer doesn't exist already!");
    }
}

pub fn list_peer() {
    let saved_peers = fetch_peer().unwrap();

    println!("Total Peers: {} ", saved_peers.len());

    for peer in saved_peers {
        println!("{:?}", peer);
    }
}

fn fetch_peer() -> Result<Vec<SavedPeer>, Box<dyn std::error::Error>> {
    let mut file = File::open(PEER_PATH).expect("Initialized Whisper");

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let saved_peers: Vec<SavedPeer> = serde_json::from_str(&contents).unwrap();

    Ok(saved_peers)
}

pub fn write_envs(peers_env_variables: HashMap<String, String>) -> Result<(), std::io::Error> {
    let mut file = File::create(ENV_PATH).expect("Unable to open or create .env File");

    let mut buffer = String::new();
    for (key, value) in peers_env_variables {
        buffer.insert_str(usize::MAX, format!("{}={}\n", key, value).as_str());
    }

    file.write_all(buffer.as_bytes())
}

pub fn read_envs() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut file = File::open(ENV_PATH).expect("Issue with opening Env file. \n Check if it exists!");

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let env_variables: HashMap<String, String> = contents.lines().filter_map(
        |line| 

        if line.trim().is_empty() {
            None
        } else {
            let iterator = line.trim().split_once("=").unwrap();
            Some((String::from(iterator.0), String::from(iterator.1)))
        }
    ).collect();
    
    Ok(env_variables)
}

// ************************* *************************
pub fn search_peer() {
    println!("Searching for peers");
}

pub fn push_envs() {
    println!("Pushing env to peers");
}

pub fn pull_envs() {
    println!("Pulling env from peers");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        init();

        assert!(std::path::Path::new(PEER_PATH).exists());
    }

    #[test]
    fn test_read_env() {
        read_envs().unwrap();

        assert!(std::path::Path::new(ENV_PATH).exists());
    }
}