use libp2p::identity::Keypair;

const SECRETS_PATH: &str = ".whisper/identity.pk8";

pub fn check_or_create_keys() -> Result<Keypair, Box<dyn std::error::Error>> {
    
    if let Ok(bytes) = std::fs::read(SECRETS_PATH) {
        Ok(Keypair::from_protobuf_encoding(&bytes)?)
    } else {
        let key = Keypair::generate_ed25519();

        let new_key = key.to_protobuf_encoding()?;

        std::fs::create_dir_all(".whisper")?;
        std::fs::write(SECRETS_PATH, new_key)?;

        Ok(key)
    }
}