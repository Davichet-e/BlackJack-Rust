use crate::{Deck, Hand};
use std::fmt;

pub struct Player {
    pub hands: (Hand, Option<Hand>),
    pub name: String,
    pub initial_money: i32,
    pub actual_money: i32,
    pub actual_bet: i32,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Player {
    pub fn new(name: String, initial_money: i32, deck: &mut Deck) -> Player {
        Player {
            hands: (Hand::new(deck), None),
            name,
            initial_money,
            actual_money: initial_money,
            actual_bet: 0,
        }
    }

    pub fn first_hand(&self) -> Hand {
        self.hands.0.clone()
    }

    pub fn hand_mut(&mut self) -> &mut Hand {
        &mut self.hands.0
    }

    pub fn bet(&mut self, money: i32) {
        self.actual_bet = money;
    }

    pub fn hit(&mut self, deck: &mut Deck, hand_index: usize) {
        if hand_index == 0 {
            self.hands.0.deal_card(deck);
        } else {
            match self.hands.1.as_mut() {
                Some(v) => v.deal_card(deck),
                None => (),
            }
        }
    }
    fn can_double(&self) -> bool {
        self.actual_bet * 2 <= self.actual_money
            && self.first_hand().cards.len() == 2
            && self.hands.1.is_none()
    }
    pub fn double(&mut self) -> bool {
        if self.can_double() {
            self.actual_bet *= 2;
            return true;
        }
        false
    }
    fn can_surrender(&self) -> bool {
        self.first_hand().cards.len() == 2 && self.hands.1.is_none()
    }

    pub fn surrender(&mut self) -> bool {
        if self.can_surrender() {
            self.actual_bet /= 2;
            self.hands.0.points = 0;
            return true;
        }
        false
    }

    fn can_split(&self) -> bool {
        self.actual_bet * 2 <= self.actual_money
            && self.hands.1.is_none()
            && self.hands.0.cards.len() == 2
            && self.hands.0.cards[0].name == self.hands.0.cards[1].name
    }
    pub fn split(&mut self, deck: &mut Deck) -> bool {
        if self.can_split() {
            self.actual_bet *= 2;
            let cards = vec![
                self.hands.0.cards.pop().expect("Failed to split"),
                deck.deal_card(),
            ];
            let points = Hand::calculate_points(&cards);
            self.hands.1 = Some(Hand {
                cards,
                points,
                aces: 0,
            });

            self.hands.0.deal_card(deck);
            return true;
        }
        false
    }
    // pub fn surrender(&mut self,     )
    pub fn win(&mut self) {
        if self.hands.1.is_some() {
            self.actual_money += self.actual_bet / 2;
        } else {
            self.actual_money += self.actual_bet;
        }
    }
    pub fn lose(&mut self) {
        if self.hands.1.is_some() {
            self.actual_money -= self.actual_bet / 2;
        } else {
            self.actual_money -= self.actual_bet;
        }
    }
}
