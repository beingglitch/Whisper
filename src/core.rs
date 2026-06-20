use std::{collections::HashMap, path::Path, time::Duration};
use futures::StreamExt;
use libp2p::{Multiaddr, StreamProtocol, Swarm, SwarmBuilder, noise, request_response::{self, ProtocolSupport}, swarm::SwarmEvent, tcp, yamux};
use serde::{Deserialize, Serialize};

use crate::handlers::{env_handler, key_handler, peer_handler::read_peers};

pub async fn sync(dialer: bool) {

    let keypair = key_handler::fetch_Identity().unwrap();

    let mut swarm: Swarm<request_response::json::Behaviour<EnvRequest, EnvResponse>> = SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default).unwrap()
        .with_behaviour(|_| request_response::json::Behaviour::new([(StreamProtocol::new("/env/1.0.0"), ProtocolSupport::Full)], request_response::Config::default())).unwrap()
        .with_swarm_config(
            |cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
        )
        .build();
    
    println!("Local PeerID: {}", swarm.local_peer_id());
    

    if dialer == false {
        println!("hi");
        swarm.listen_on("/ip4/0.0.0.0/tcp/14550".parse().unwrap()).unwrap();
    } else {
        println!("hi2");
        let addr: Multiaddr = "/ip4/127.0.0.1/tcp/14550".parse().unwrap();
        swarm.dial(addr).unwrap();
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { listener_id, address } => { println!("listener_id: {listener_id} address: {address}")},
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                println!("Connection Established! peer_id: {peer_id}");
                if dialer {
                    println!("hi");
                    swarm.behaviour_mut().send_request(&peer_id, EnvRequest {});
                }
            },
            SwarmEvent::Behaviour( request_response::Event::Message { 
                message: request_response::Message::Request { channel, .. }, peer }) => {

                    let peer_config = read_peers();
                    
                    if !peer_config.peers.iter().any(|p| p.peer_id == peer.to_string()) {

                        println!{"Peer {peer} is not allowed to sync env! Add it via wishper peer add {peer} to give permission"};

                        let _ = swarm.behaviour_mut().send_response(
                            channel, 
                            EnvResponse { env_variables: HashMap::default(), message: "No permission to fetch".to_string() }
                        );
                        continue;
                    }
                    
                    let env_path = ".env";
                    if let Some(env_variables) = env_handler::read_env(Path::new(&env_path)) {
                        let _ = swarm.behaviour_mut().send_response(channel, EnvResponse { env_variables: env_variables, message: "".to_string()});
                    };
                    break;
                },
            SwarmEvent::Behaviour( request_response::Event::Message {
                message: request_response::Message::Response { response, .. }, ..}) => {
                    let env_path = ".env";
                    env_handler::sync_env(Path::new(&env_path), response.env_variables).unwrap();
                    println!("Env Synced!");
                    break;
                },
            
            _ => {}
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EnvRequest {}
#[derive(Debug, Serialize, Deserialize)]
struct EnvResponse {
    env_variables: HashMap<String, String>,
    message: String
}