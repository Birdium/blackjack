use crate::game::Game;
use crate::card::Card;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tarpc::context;
use tarpc::serde_transport::tcp;
use tarpc::server::{self, Channel};
use tokio_serde::formats::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub player_hand: Vec<Card>,
    pub dealer_hand: Vec<Card>,
    pub result: Option<String>,
}

#[tarpc::service]
pub trait Blackjack {
    async fn new_game() -> GameState;
    async fn hit() -> GameState;
    async fn stand() -> GameState;
}

#[derive(Clone)]
pub struct BlackjackServer {
    game: Arc<Mutex<Game>>,
}

#[tarpc::server]
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

impl BlackjackServer {
    pub async fn serve(self, addr: &str) -> anyhow::Result<()> {
        let listener = tcp::listen(addr, Json::default).await?;
        listener
            .filter_map(|r| async { r.ok() })
            .map(BaseChannel::with_defaults)
            .map(|channel| {
                let server = self.clone();
                channel.execute(server.serve())
            })
            .buffer_unordered(10)
            .for_each(|_| async {})
            .await;
        Ok(())
    }
}