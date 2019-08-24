use std::fmt;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Card {
    pub value: i8,
    suit: char,
}

impl Card {
    pub fn value_to_name(&self) -> &str {
        match self.value {
            11 => "ACE",
            2 => "TWO",
            3 => "THREE",
            4 => "FOUR",
            5 => "FIVE",
            6 => "SIX",
            7 => "SEVEN",
            8 => "EIGHT",
            9 => "NINE",
            10 => "TEN",
            12 => "JACK",
            13 => "QUEEN",
            14 => "KING",
            _ => panic!("Not valid"),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.value_to_name(), self.suit)
    }
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        const SUITS: [char; 4] = ['♥', '♦', '♣', '♠'];
        let mut deck: Vec<Card> = Vec::new();

        for &suit in SUITS.iter() {
            for card_num in 2..=14 {
                let card_value = if card_num < 12 { card_num } else { 10 };
                deck.push(Card {
                    value: card_value,
                    suit: suit,
                });
            }
        }
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);

        Deck { cards: deck }
    }

    pub fn deal_card(&mut self) -> Card {
        if self.cards.is_empty() {
            self.initialize_deck()
        }
        self.cards.pop().expect("DECK EMPTY!")
    }

    pub fn get_initial_cards(&mut self) -> Vec<Card> {
        vec![self.deal_card(), self.deal_card()]
    }

    fn initialize_deck(&mut self) {
        *self = Deck::new();
    }
}
