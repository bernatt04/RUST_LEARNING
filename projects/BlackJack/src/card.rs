pub enum Suit {
    //enum es solo para decir que suit es un tipo nuevo que solo puede valer estas 4 cosas.
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

pub struct Card {
    //Constructor de una card nos dice como construir una card mas adelante le tenemos que asignar un valor para suit y para rank.
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        //create and return card
        Card {
            suit: suit,
            rank: rank,
        }
    }
    pub fn value(&self) -> u8 {
        //match rank and return value
        //simplemente se le da valor a cada uno de los ranks
        match self.rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 11,
        }
    }
}
