mod handlers;
mod cli;

use handlers::key_handler;
use clap::Parser;
use handlers::peer_handler;

fn main() {
    let args = cli::Args::parse();
    match args.command {
        cli::Commands::Init => key_handler::init(),
        cli::Commands::Id => key_handler::print_key(),
        cli::Commands::Status => println!("status"),
        cli::Commands::Sync => println!("sync"),
        cli::Commands::Peer { command } => match command {
            cli::PeerCommands::Add { peer_id } => peer_handler::add_peer(peer_id),
            cli::PeerCommands::Remove { peer_id } => peer_handler::remove_peer(peer_id),
            cli::PeerCommands::List => peer_handler::list_peers(),
        }
        _ => {println!("print something main")}
    }
}