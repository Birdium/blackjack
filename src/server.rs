mod card;
mod deck;
mod game;
mod player;
mod rpc;

use rpc::BlackjackServer;
use tarpc::serde_transport::tcp::listen;
use tarpc::server::{Channel, BaseChannel};
use tarpc::tokio_serde::formats::Json;
use futures::prelude::*;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use crate::game::Game;
use crate::rpc::Blackjack;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {tokio::spawn(fut);}
    let game = Arc::new(Mutex::new(Game::new()));
    let server = BlackjackServer { game };

    let addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), 11451);

    let mut listener = listen(&addr, Json::default).await?;
    listener.config_mut().max_frame_length(usize::MAX);
    listener
        .filter_map(|r| async { r.ok() })
        .map(BaseChannel::with_defaults)
        .map(|channel| {
            let server = server.clone();
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;
    Ok(())
}