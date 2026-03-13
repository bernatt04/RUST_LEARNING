//trying to do the card file by myself without looking at the file.

//i am sorry to inform that i will be commenting my whole thought process so if i come back i can remember later.

//so, i start by thinking, what is a card? a card is just a construct of a numbre 1-13 and a suit, (hearts, diamonds...)
//so basically i am going to start with building suits and ranks.
enum Suit {
    Diamonds,
    Spades,
    Clubs,
    Hearts,
}

enum Rank {
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

//so a card is just basically a structure of both a suit and a rank
struct Card {
    palo: Suit, //written in spanish so i can have a distinction in the new() func.
    numero: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            palo: suit,
            numero: rank,
        }
    }

    pub fn value(&self) -> u32 {
        match self.numero {
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
