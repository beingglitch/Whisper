use std::fs;

use serde::{Deserialize, Serialize};
use toml;

const PEERS_PATH: &str = ".whisper/peers.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Peer {
    pub peer_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeersConfig {
    pub peers: Vec<Peer>,
}

pub fn read_peers() -> PeersConfig {
    let Ok(data) = fs::read_to_string(PEERS_PATH) else {
        return PeersConfig { peers: vec![]};
    };

    toml::from_str(&data).unwrap_or(PeersConfig { peers: vec![]})
}

fn write_peers(peers_config: PeersConfig) {
    let data = toml::to_string(&peers_config).unwrap();
    std::fs::write(PEERS_PATH, data).unwrap();
}

pub fn check_peer(peer_id: &str, config: &PeersConfig) -> bool {
    config.peers.iter().any(|p| p.peer_id == peer_id)
}

pub fn add_peer(peer_id: String) {
    let mut peers_config = read_peers();
    if check_peer(&peer_id, &peers_config) == true {
        println!("Peer {peer_id} already exist!");
        return;
    };
    peers_config.peers.push(Peer {peer_id});
    write_peers(peers_config);
}

pub fn remove_peer(peer_id: String) {
    let mut peers_config = read_peers();
    peers_config.peers.retain(|p| p.peer_id != peer_id);
    write_peers(peers_config);
}

pub fn list_peers() {
    let peers_config = read_peers();

     if peers_config.peers.is_empty() {
        println!("No peers configured. Use whisper peer add <peer_id>");
        return;
    }
    
    println!("Total Peers: {}", peers_config.peers.len());
    for peer in &peers_config.peers {
        println!("{}", peer.peer_id);
    }
}