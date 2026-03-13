use crate::card::{Card, Rank, Suit};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();

        //even though we already did the following in the card file the difference is the following:
        //in the card.rs file we created the types, which means a suit for example can only be values spades, clubs...
        //but here we have to declare them because they were not created prior to this.
        let suits = [Suit::Diamonds, Suit::Spades, Suit::Clubs, Suit::Hearts];

        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];

        for suit in &suits {
            for rank in &ranks {
                cards.push(Card::new(*suit, *rank));
            }
        }

        Deck { cards } //this creates the deck
    }
}
