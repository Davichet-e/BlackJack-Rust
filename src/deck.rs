use std::fmt;

use rand;
use rand::seq::SliceRandom;

#[derive(PartialEq, Debug, Clone)]
pub struct Card {
    pub name: String,
    suit: char,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.name, self.suit)
    }
}

impl Card {
    pub fn name_to_value(&self) -> u8 {
        match self.name.to_uppercase().trim() {
            "ACE" => 11,
            "TWO" => 2,
            "THREE" => 3,
            "FOUR" => 4,
            "FIVE" => 5,
            "SIX" => 6,
            "SEVEN" => 7,
            "EIGHT" => 8,
            "NINE" => 9,
            "TEN" | "JACK" | "QUEEN" | "KING" => 10,
            _ => panic!("Name not valid."),
        }
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(n_decks: u8) -> Deck {
        let suits: [char; 4] = ['♥', '♦', '♣', '♠'];

        let card_names: [&str; 13] = [
            "ACE", "TWO", "THREE", "FOUR", "FIVE", "SIX", "SEVEN", "EIGHT", "NINE", "TEN", "JACK",
            "QUEEN", "KING",
        ];

        let mut deck: Vec<Card> = Vec::new();
        for _ in 0..n_decks {
            for &suit in &suits {
                for &card_name in &card_names {
                    deck.push(Card {
                        name: card_name.to_string(),
                        suit,
                    });
                }
            }
        }
        let mut rng = rand::thread_rng();
        deck.shuffle(&mut rng);

        Deck { cards: deck }
    }

    pub fn deal_card(&mut self) -> Card {
        self.cards.pop().expect("DECK EMPTY, GAME OVER!")
    }

    pub fn get_initial_cards(&mut self) -> Vec<Card> {
        vec![self.deal_card(), self.deal_card()]
    }
}
