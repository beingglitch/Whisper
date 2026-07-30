use clap::{Parser, Subcommand};
use whisper_core;

#[derive(Parser, Debug)]
struct Args {

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    Push,
    Pull,

    #[command(subcommand)]
    Peer(PeerCommands),
}

#[derive(Subcommand, Debug)]
enum PeerCommands {
    Add(PeerArgs),
    Remove(PeerArgs),
    List
}

#[derive(clap::Args, Debug)]
struct PeerArgs {
    peer_id: String,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Init => {
            whisper_core::init();
        }
        Commands::Push => {
            whisper_core::push_envs();
        }
        Commands::Pull => {
            whisper_core::pull_envs();
        }
        Commands::Peer(peer_command) => match peer_command {
            PeerCommands::Add(PeerArgs { peer_id }) => {
                whisper_core::add_peer(peer_id);
            }
            PeerCommands::Remove(PeerArgs { peer_id }) => {
                whisper_core::remove_peer(peer_id);
            }
            PeerCommands::List => {
                whisper_core::list_peer();
            }
        },
    }
}