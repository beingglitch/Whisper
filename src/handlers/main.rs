mod handlers;

use std::{collections::HashMap, path::Path, time::Duration};
use futures::StreamExt;
use libp2p::{Multiaddr, StreamProtocol, Swarm, SwarmBuilder, noise, request_response::{self, ProtocolSupport}, swarm::SwarmEvent, tcp, yamux};
use serde::{Deserialize, Serialize};

use handlers::{env_handler, key_handler};

#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let keypair = key_handler::check_or_create_keys()?;

    let mut swarm: Swarm<request_response::json::Behaviour<EnvRequest, EnvResponse>> = SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)?
        .with_behaviour(|_| request_response::json::Behaviour::new([(StreamProtocol::new("/env/1.0.0"), ProtocolSupport::Full)], request_response::Config::default()))?
        .with_swarm_config(
            |cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
        )
        .build();
    
    println!("Local PeerID: {}", swarm.local_peer_id());
    
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;


    let mut dialer = false;
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;

        swarm.dial(remote)?;
        dialer = true;
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { listener_id, address } => { println!("listener_id: {listener_id} address: {address}")},
            SwarmEvent::ConnectionEstablished { peer_id, connection_id, endpoint, num_established, concurrent_dial_errors, established_in } => {
                println!("Connection Established! peer_id: {peer_id}");
                if dialer {
                    println!("hi");
                    swarm.behaviour_mut().send_request(&peer_id, EnvRequest {});
                }
            },
            SwarmEvent::Behaviour( request_response::Event::Message { 
                message: request_response::Message::Request { request, channel, .. }, .. }) => {
                    let env_path = ".env";
                    if let Some(env_variables) = env_handler::read_env(Path::new(&env_path)) {
                        let _ = swarm.behaviour_mut().send_response(channel, EnvResponse { env_variables: env_variables});
                    };
                },
            SwarmEvent::Behaviour( request_response::Event::Message {
                message: request_response::Message::Response { request_id, response }, ..}) => {
                    let env_path = ".env";
                    env_handler::sync_env(Path::new(&env_path), response.env_variables)?;
                    println!("Env Synced!")
                },
            
            _ => {}
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EnvRequest {}
#[derive(Debug, Serialize, Deserialize)]
struct EnvResponse {
    env_variables: HashMap<String, String>
}