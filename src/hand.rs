use crate::deck::{Card, Deck};

use std::fmt;

#[derive(Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub points: u8,
    aces: u8,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let points: u8 = self.points;
        let cards_as_string: Vec<String> = self.cards.iter().map(|card| card.to_string()).collect();
        let (last, elements): (&String, &[String]) =
            cards_as_string.split_last().expect("Hand was empty.");

        write!(
            f,
            "{} and {} ({} points)",
            elements.join(", "),
            last,
            if points > 0 {
                points.to_string()
            } else {
                String::from("> 21")
            }
        )
    }
}

impl Hand {
    /// Creates a new instance of a `Hand` given a `Deck` to get the initial cards.
    pub fn new(deck: &mut Deck) -> Hand {
        let cards: Vec<Card> = deck.get_initial_cards();
        let points: u8 = Hand::calculate_points(&cards);

        let mut hand = Hand {
            cards,
            points,
            aces: 0,
        };
        for card in hand.cards.clone() {
            hand.check_if_ace(&card);
        }
        hand.check_ace_points();
        hand
    }
    /// Creates a new instance of a `Hand` given a slice of `Card`s
    pub fn from_cards(cards: &[Card]) -> Hand {
        let points: u8 = Hand::calculate_points(cards);

        let mut hand = Hand {
            cards: cards.into(),
            points,
            aces: 0,
        };
        for card in hand.cards.clone() {
            hand.check_if_ace(&card);
        }
        hand.check_ace_points();
        hand
    }

    pub fn has_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.points == 21
    }

    /// Resets all the attributes of the instance.
    pub fn initialize_attributes(&mut self, deck: &mut Deck) {
        *self = Hand::new(deck);
    }

    /// Deal a new `Card`, taken from the `Deck` given as a parameter.
    pub fn deal_card(&mut self, deck: &mut Deck) {
        let card: Card = deck.deal_card();
        self.check_if_ace(&card);

        self.cards.push(card);
        self.update_points();

        if self.points > 21 {
            self.points = 0;
        }
    }

    fn check_if_ace(&mut self, card: &Card) {
        if card.name == "ACE" {
            self.aces += 1;
        }
    }

    fn check_ace_points(&mut self) {
        while self.aces > 0 && self.points > 21 {
            self.points -= 10;
            self.aces -= 1;
        }
    }

    fn update_points(&mut self) {
        self.points = Hand::calculate_points(&self.cards);
        self.check_ace_points();
    }

    pub fn calculate_points(cards: &[Card]) -> u8 {
        cards.iter().fold(0, |acc, card| acc + card.name_to_value())
    }
}
