mod handlers;

use std::{time::Duration, error::Error};

use serde::{Serialize, Deserialize};
use futures::StreamExt;
use libp2p::{Multiaddr, StreamProtocol, Swarm, noise, request_response::{self, Config, ProtocolSupport}, swarm::SwarmEvent, tcp, yamux};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut swarm: Swarm<request_response::json::Behaviour<EnvRequest, EnvResponse>> = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)?
        .with_behaviour(|_| request_response::json::Behaviour::new([(StreamProtocol::new("/env/1.0.0"), ProtocolSupport::Full)], Config::default()))?
        .with_swarm_config(|cfg| 
            cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
        )
        .build();

    
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let mut dialer = false;
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        dialer = true;
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { listener_id, address } => { println!("{listener_id}, {address}")},
            // SwarmEvent::Behaviour(event) => { println!("{event:?}")},
            SwarmEvent::ConnectionEstablished { peer_id, .. } => { 
                println!("Connection established with {peer_id}");
                if dialer {
                    println!("hi");
                    swarm.behaviour_mut().send_request(&peer_id, EnvRequest {});
                };
            },   
            SwarmEvent::Behaviour(request_response::Event::Message {
                message: request_response::Message::Request { request, channel, ..}, ..
            }) => { 
                println!("recieved {request:?}"); 
                let env_variables = handlers::env_handler::parse_and_return(".env")?;
                let _ = swarm.behaviour_mut().send_response(channel, EnvResponse { 
                        env_variables
                    }); 
                },
            SwarmEvent::Behaviour(request_response::Event::Message {
                message: request_response::Message::Response { response, .. }, ..
            }) => { println!("recieved {response:?}");},
            _ => {} 
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EnvRequest {}
#[derive(Debug, Serialize, Deserialize)]
struct EnvResponse {
    env_variables: std::collections::HashMap<String, String>
}