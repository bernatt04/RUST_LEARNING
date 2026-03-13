use crate::card::Card;

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    pub fn addCard(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn calculate_score_player() {
        let mut score = 0;
        let mut aces = 0;
    }
}
