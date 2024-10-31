use service::{Card, Rank};

pub struct Player {
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
        Self { hand: Vec::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn hand_value(&self) -> u8 {
        let mut value = 0;
        let mut aces = 0;

        for card in &self.hand {
            value += card.value();
            if let Rank::Ace = card.rank {
                aces += 1;
            }
        }

        while value > 21 && aces > 0 {
            value -= 10;
            aces -= 1;
        }

        value
    }

    pub fn is_busted(&self) -> bool {
        self.hand_value() > 21
    }
}