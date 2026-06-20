use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {

    #[command(subcommand)]
    pub command: Commands

}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,

    Id,

    Status,

    Sync {
       #[command(subcommand)]
        command: SyncCommands,
    },

    Peer {
        #[command(subcommand)]
        command: PeerCommands
    }
}

#[derive(Subcommand, Debug)]
pub enum PeerCommands {
    Add { peer_id: String },

    Remove { peer_id: String },

    List
}

#[derive(Subcommand, Debug)]
pub enum SyncCommands {
    /// Push local .env to peers
    Push,
    /// Pull .env from peers
    Pull,
}