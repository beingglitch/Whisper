use libp2p::identity::Keypair;

const SECRETS_PATH: &str = ".whisper/identity.pk8";

pub fn fetch_identity() -> Option<Keypair> {
    
    if let Ok(bytes) = std::fs::read(SECRETS_PATH) {
        let keypair = Keypair::from_protobuf_encoding(&bytes).unwrap();
        return Some(keypair);
    }

    println!("Project not Intiated! Use \"whisper init\" ");
    None
}

pub fn init() {
    
    let _ = std::fs::create_dir_all(".whisper"); // TODO

    if let Ok(bytes) = std::fs::read(SECRETS_PATH) {
        let keypair = Keypair::from_protobuf_encoding(&bytes).unwrap();

        let peer_id = libp2p::PeerId::from(keypair.public());
        println!("Identity already exist: {peer_id}");

    } else {
        let key = Keypair::generate_ed25519();

        let new_key = key.to_protobuf_encoding().unwrap();

        std::fs::create_dir_all(".whisper").unwrap();
        std::fs::write(SECRETS_PATH, new_key.clone()).unwrap();

        let peer_id = libp2p::PeerId::from(&key.public());
        println!("Generated new identity: {peer_id}");
    }
}

pub fn print_key() {
    if let Ok(bytes) = std::fs::read(SECRETS_PATH) {
        let keypair = Keypair::from_protobuf_encoding(&bytes).unwrap();

        let peer_id = libp2p::PeerId::from(keypair.public());
        println!("{peer_id}");
    } else {
       println!("No Key Found!");
    }
}