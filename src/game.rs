// src/game.rs
use crate::deck::Deck;
use crate::player::Player;

pub struct Game {
    pub deck: Deck,
    pub player: Player,
    pub dealer: Player,
}

impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        Self {
            deck,
            player: Player::new(),
            dealer: Player::new(),
        }
    }

    pub fn deal_initial_cards(&mut self) {
        for _ in 0..2 {
            self.player.add_card(self.deck.deal().unwrap());
            self.dealer.add_card(self.deck.deal().unwrap());
        }
    }

    pub fn player_hit(&mut self) {
        self.player.add_card(self.deck.deal().unwrap());
    }

    pub fn dealer_turn(&mut self) {
        while self.dealer.hand_value() < 17 {
            self.dealer.add_card(self.deck.deal().unwrap());
        }
    }

    pub fn determine_winner(&self) -> &str {
        if self.player.is_busted() {
            "Dealer wins"
        } else if self.dealer.is_busted() {
            "Player wins"
        } else if self.player.hand_value() > self.dealer.hand_value() {
            "Player wins"
        } else if self.player.hand_value() < self.dealer.hand_value() {
            "Dealer wins"
        } else {
            "Push"
        }
    }
}