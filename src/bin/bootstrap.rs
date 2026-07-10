use futures::StreamExt;
use libp2p::{SwarmBuilder, identity::Keypair, kad, noise, swarm::SwarmEvent, tcp, yamux};

const SECRETS_PATH: &str = ".whisper/bootstrap/identity.pk8";

#[tokio::main]
async fn main() {

    let keypair = if let Ok(bytes) = std::fs::read(SECRETS_PATH) {
        Keypair::from_protobuf_encoding(&bytes).unwrap()
    } else {
        let key = Keypair::generate_ed25519();
        std::fs::create_dir_all(".whisper/bootstrap").unwrap();
        std::fs::write(SECRETS_PATH, key.to_protobuf_encoding().unwrap()).unwrap();
        key
    };

    let mut swarm = SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default).unwrap()
        .with_behaviour(|key| kad::Behaviour::new(
            key.public().to_peer_id(),
            kad::store::MemoryStore::new(key.public().to_peer_id())
        )).unwrap()
        .build();

    
    println!("Bootstrap PeerID: {}", swarm.local_peer_id());
    
    swarm.behaviour_mut().set_mode(Some(kad::Mode::Server));
    
    swarm.listen_on("/ip4/0.0.0.0/tcp/14550".parse().unwrap()).unwrap();

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::ConnectionEstablished { peer_id, connection_id, .. } => {
                println!("Connection established with connection id: {} & peer id: {}", connection_id, peer_id);
            },
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Bootstrap listening on {address}");
            }
            _ => {}
        }
    }
}