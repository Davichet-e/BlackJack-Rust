use std::fmt;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(PartialEq, Debug, Clone)]
pub struct Card {
    pub name: String,
    suit: char,
}

impl Card {
    pub fn name_to_value(&self) -> i8 {
        match self.name.as_str() {
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
            _ => panic!("Not valid"),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.name, self.suit)
    }
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(n_decks: u8) -> Deck {
        const SUITS: [char; 4] = ['♥', '♦', '♣', '♠'];
        const CARD_NAMES: [&str; 13] = [
            "ACE", "TWO", "THREE", "FOUR", "FIVE", "SIX", "SEVEN", "EIGHT", "NINE", "TEN", "JACK",
            "QUEEN", "KING",
        ];
        let mut deck: Vec<Card> = Vec::new();
        for _ in 0..n_decks {
            for &suit in SUITS.iter() {
                for &card_name in CARD_NAMES.iter() {
                    deck.push(Card {
                        name: card_name.to_string(),
                        suit: suit,
                    });
                }
            }
        }
        let mut rng = thread_rng();
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
