use std::{io, process::exit};

mod card;
mod card2;
mod deck;
mod game;

// Instructions are as follows:
// 1. Two players (Player and Dealer) and only one deck.
// 2. Initially, 2 cards are dealt to each hand and the score is calculated.
// 3. The system asks if the player wants to [H] Hit or [S] Stay.
// 4. If Hit: a card is dealt to the player. If score > 21, the player busts (loses).
// 5. If Stay: it's the dealer's turn. The dealer must hit until their score is at least 17.
// 6. Finally, compare scores: the one closer to 21 without going over wins.
fn main() {
    println!("hello Welcome to blackjack");
    while true {
        println!("Type 1 to start the game or 2 to exit the game");

        //simply choice of game
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap();

        //simple structure
        if choice == 1 {
            //declaring players and deck
            let mut player = game::Hand::new();
            let mut dealer = game::Hand::new();
            let mut baraja = deck::Deck::new();

            baraja.shuffle();

            //dealing the first hands
            let mut carta = baraja.deal();
            player.add_card(carta);
            let mut carta = baraja.deal();
            dealer.add_card(carta);
            let mut carta = baraja.deal();
            player.add_card(carta);
            let mut carta = baraja.deal();
            dealer.add_card(carta);

            let mut score_player = game::Hand::calc_score(&player);
            let mut score_dealer = game::Hand::calc_score(&dealer);

            println!("the dealer's score is {}", score_dealer);
            println!("the player's score is {}", score_player);

            println!("Hit or Stay? type h or s");
            let mut election = String::new();
            io::stdin().read_line(&mut election).unwrap();
            let election = election.trim();

            if election == "h" {
                let mut carta = baraja.deal();
                player.add_card(carta);
                if player.calc_score() > 21 {
                    println!("your score is {}, you lost", player.calc_score());
                } else {
                    println!("your score is {}", player.calc_score());
                    println!("Hit or Stay? type h or s");

                    let mut election = String::new();
                    io::stdin().read_line(&mut election).unwrap();
                    election.trim();
                }
            } else {
                if score_dealer < 17 {
                    let carta = baraja.deal();
                    dealer.add_card(carta);
                    score_dealer = dealer.calc_score();
                    if score_dealer > 21 {
                        println!("you won, dealer busted");
                    } else {
                        if score_dealer > score_player {
                            println!("dealer won");
                        } else if score_dealer == score_player {
                            println!("draw");
                        } else {
                            println!("you won!!!!!");
                        }
                    }
                }
            }
        } else {
            exit(0);
        }
    }
}
