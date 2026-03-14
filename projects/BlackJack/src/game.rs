use crate::card::Card;

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    //create new hand.
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    //add a card to the current hand
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn calc_score(&self) -> u8 {
        let mut score = 0;

        for card in &self.cards {
            score += card.value();
        }
        return score;
    }
}
