mod handlers;
mod cli;
mod core;

use handlers::key_handler;
use clap::Parser;
use handlers::peer_handler;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    match args.command {
        cli::Commands::Init => key_handler::init(),
        cli::Commands::Id => key_handler::print_key(),
        cli::Commands::Status => println!("status"),
        cli::Commands::Sync { command} => match command {
            cli::SyncCommands::Pull => core::sync(true).await,
            cli::SyncCommands::Push => core::sync(false).await
        },
        cli::Commands::Peer { command } => match command {
            cli::PeerCommands::Add { peer_id } => peer_handler::add_peer(peer_id),
            cli::PeerCommands::Remove { peer_id } => peer_handler::remove_peer(peer_id),
            cli::PeerCommands::List => peer_handler::list_peers(),
        }
    }
}