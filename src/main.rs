mod card;
mod deck;
mod game;
mod player;
mod rpc;

use rpc::BlackjackServer;
use std::sync::{Arc, Mutex};

fn main() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        let game = Arc::new(Mutex::new(game::Game::new()));
        let server = BlackjackServer { game };

        server.serve("127.0.0.1:50051").await
    })
}