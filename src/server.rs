mod deck;
mod game;
mod player;

use tarpc::serde_transport::tcp::listen;
use tarpc::server::{Channel, BaseChannel};
use tarpc::tokio_serde::formats::Json;
use tarpc::context;
use futures::prelude::*;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use crate::game::Game;
use service::{Blackjack, GameState};


#[derive(Clone)]
pub struct BlackjackServer {
    pub game: Arc<Mutex<Game>>,
}

// #[tarpc::server]
impl Blackjack for BlackjackServer {
    async fn new_game(self, _: context::Context) -> GameState {
        let mut game = self.game.lock().unwrap();
        *game = Game::new();
        game.deal_initial_cards();
        GameState {
            player_hand: game.player.hand.clone(),
            dealer_hand: vec![game.dealer.hand[0]],
            result: None,
        }
    }

    async fn hit(self, _: context::Context) -> GameState {
        let mut game = self.game.lock().unwrap();
        game.player_hit();
        let result = if game.player.is_busted() {
            Some("Dealer wins".to_string())
        } else {
            None
        };
        GameState {
            player_hand: game.player.hand.clone(),
            dealer_hand: vec![game.dealer.hand[0]],
            result,
        }
    }

    async fn stand(self, _: context::Context) -> GameState {
        let mut game = self.game.lock().unwrap();
        game.dealer_turn();
        let result = Some(game.determine_winner().to_string());
        GameState {
            player_hand: game.player.hand.clone(),
            dealer_hand: game.dealer.hand.clone(),
            result,
        }
    }
}


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