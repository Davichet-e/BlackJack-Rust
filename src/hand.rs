use crate::{Card, Deck};
use std::fmt;

pub struct Hand {
    pub cards: Vec<Card>,
    pub points: i8,
    aces: i8,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.cards
                .iter()
                .map(|card| card.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Hand {
    pub fn new(deck: &mut Deck) -> Hand {
        let cards: Vec<Card> = deck.get_initial_cards();
        let points: i8 = Hand::calculate_points(&cards);

        let mut hand = Hand {
            cards: cards,
            points: points,
            aces: 0,
        };
        for card in hand.cards.clone() {
            hand.check_if_ace(card);
        }
        hand.check_ace_points();
        hand
    }

    pub fn initialize_attributes(&mut self, deck: &mut Deck) {
        *self = Hand::new(deck);
    }

    pub fn deal_card(&mut self, deck: &mut Deck) {
        let card: Card = deck.deal_card();
        self.check_if_ace(card);
        self.cards.push(card);
        self.update_points(card);
        if self.check_if_lost() {
            self.points = 0;
        }
    }

    fn check_if_ace(&mut self, card: Card) {
        if card.value == 11 {
            self.aces += 1;
        }
    }

    fn check_if_lost(&self) -> bool {
        self.points > 21
    }

    fn check_ace_points(&mut self) {
        while self.aces > 0 && self.check_if_lost() {
            self.points -= 10;
            self.aces -= 1;
        }
    }

    fn update_points(&mut self, card: Card) {
        self.points += card.value;
        self.check_ace_points();
    }

    pub fn calculate_points(cards: &Vec<Card>) -> i8 {
        cards.iter().fold(0, |acc, card| acc + card.value)
    }
}
