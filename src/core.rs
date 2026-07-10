use std::{collections::HashMap, path::Path, time::Duration};
use futures::StreamExt;
use libp2p::{Multiaddr, StreamProtocol, Swarm, SwarmBuilder, kad, multiaddr::Protocol, noise, request_response::{self, ProtocolSupport}, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux};
use serde::{Deserialize, Serialize};

use crate::handlers::{env_handler, key_handler, peer_handler::read_peers};

pub async fn sync(dialer: bool) {

    let keypair = key_handler::fetch_identity().unwrap();

    let mut swarm: Swarm<WhisperBehaviour> = SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default).unwrap()
        .with_dns().unwrap()
        .with_behaviour(|key| {
            let kademlia = kad::Behaviour::new(
                key.public().to_peer_id(),
                kad::store::MemoryStore::new(key.public().to_peer_id())
            );
            let request_response = request_response::json::Behaviour::new(
                [(StreamProtocol::new("/env/1.0.0"), ProtocolSupport::Full)],
                request_response::Config::default()
            );

            WhisperBehaviour {request_response, kademlia}
    
        }).unwrap()
        .with_swarm_config(
            |cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
        )
        .build();
    
    
    println!("Local PeerID: {}", swarm.local_peer_id());
    
    swarm.behaviour_mut().kademlia.set_mode(Some(kad::Mode::Server));

    let bootstrap_nodes = [
        "/ip4/127.0.0.1/tcp/14550/p2p/12D3KooWDhxwWQiKBGmYHaY4AwLyzy4hHeemQ24p6RWVUp9uZHDB",
    ];
    
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();


    for addr in bootstrap_nodes {
        let parsed_addr: Multiaddr = addr.parse().unwrap();

        let peer_id = parsed_addr.iter().find_map(|p| match p {
            Protocol::P2p(id) => Some(id),
            _ => None
        }).unwrap();

        swarm.behaviour_mut().kademlia.add_address(&peer_id, parsed_addr.clone());

        swarm.dial(parsed_addr).unwrap();
    }
    
    let mut has_boostraped = false;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { listener_id, address } => { println!("listener_id: {listener_id} address: {address}")},
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                println!("Connection Established! peer_id: {peer_id}");
                if !has_boostraped {
                    let _ = swarm.behaviour_mut().kademlia.bootstrap();
                    has_boostraped = true;
                }

                let peer_config = read_peers();
                let is_known = peer_config.peers.iter().any(|p| p.peer_id == peer_id.to_string());

                if dialer && is_known {
                    swarm.behaviour_mut().request_response.send_request(&peer_id, EnvRequest {});
                }
            },
            SwarmEvent::Behaviour( WhisperBehaviourEvent::RequestResponse(request_response::Event::Message { 
                message: request_response::Message::Request { channel, .. }, peer })) => {

                    let peer_config = read_peers();
                    
                    if !peer_config.peers.iter().any(|p| p.peer_id == peer.to_string()) {

                        println!{"Peer {peer} is not allowed to sync env! Add it via wishper peer add {peer} to give permission"};

                        let _ = swarm.behaviour_mut().request_response.send_response(
                            channel, 
                            EnvResponse { env_variables: HashMap::default(), message: "No permission to fetch".to_string() }
                        );
                        continue;
                    }
                    
                    let env_path = ".env";
                    if let Some(env_variables) = env_handler::read_env(Path::new(&env_path)) {
                        let _ = swarm.behaviour_mut().request_response.send_response(channel, EnvResponse { env_variables: env_variables, message: "".to_string()});
                    };
                    break;
                },
            SwarmEvent::Behaviour( WhisperBehaviourEvent::RequestResponse(
                request_response::Event::Message {
                message: request_response::Message::Response { response, .. }, ..}
            )) => {
                    let env_path = ".env";
                    env_handler::sync_env(Path::new(&env_path), response.env_variables).unwrap();
                    println!("Env Synced!");
                    break;
                },
            SwarmEvent::Behaviour(WhisperBehaviourEvent::Kademlia(
                kad::Event::RoutingUpdated { .. }
            )) => {
                println!("Routing table updated")
            },
            SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                println!("Dial failed: {peer_id:?} — {error:?}");
            },
            SwarmEvent::Behaviour(WhisperBehaviourEvent::Kademlia(
                kad::Event::OutboundQueryProgressed { result, .. }
            )) => {
                println!("Query progressed: {result:?}");

                if let kad::QueryResult::GetClosestPeers(Ok(ok)) = &result {
                    for peer in &ok.peers {
                        let _ = swarm.dial(peer.peer_id);
                    }
                }

                if let kad::QueryResult::Bootstrap(Ok(kad::BootstrapOk {..})) = result {
                    if dialer {
                        let peer_config = read_peers();
                        for p in &peer_config.peers {
                            if let Ok(target) = p.peer_id.parse::<libp2p::PeerId>() {
                                swarm.behaviour_mut().kademlia.get_closest_peers(target);
                            }
                        }
                    }
                }
            }
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

#[derive(NetworkBehaviour)]
struct WhisperBehaviour {
    request_response: request_response::json::Behaviour<EnvRequest, EnvResponse>,
    kademlia: kad::Behaviour<kad::store::MemoryStore>
}