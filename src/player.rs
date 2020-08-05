use crate::deck::{Card, Deck};
use crate::hand::Hand;

use std::fmt;

#[derive(Clone)]
pub struct Player {
    pub hands: (Hand, Option<Hand>),
    pub name: String,
    pub initial_money: u32,
    pub actual_money: u32,
    pub bet: u32,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Player {
    /// Create a new instance of a `Player`,
    /// given a `name`, a `initial_money`,
    /// and a `deck` to create the `hands` field
    pub fn new(name: String, initial_money: u32, deck: &mut Deck) -> Player {
        Player {
            hands: (Hand::new(deck), None),
            name,
            initial_money,
            actual_money: initial_money,
            bet: 0,
        }
    }
    /// Reset the attributes of the `Hand`s of the instance.
    pub fn reset_hands(&mut self, deck: &mut Deck) {
        self.hands.0.initialize_attributes(deck);
        if self.hands.1.is_some() {
            self.hands.1.as_mut().unwrap().initialize_attributes(deck);
        }
    }

    pub fn bet(&mut self, money: u32) {
        self.bet = money;
    }

    pub fn hit(&mut self, deck: &mut Deck, hand_index: usize) {
        if hand_index == 0 {
            self.hands.0.deal_card(deck);
        } else {
            self.hands.1.as_mut().unwrap().deal_card(deck);
        }
        println!("{}", self.hands.0);
    }

    /// Double the player's bet if applicable, return an error message otherwise
    pub fn double(&mut self) -> Option<&str> {
        if self.bet * 2 > self.actual_money {
            Some("Cannot double because you have not enough money!")
        } else if self.hands.0.cards.len() > 2 {
            Some("Cannot double because you have already hit!")
        } else {
            if self.hands.1.is_some() {
                self.bet += self.bet / 2;
            } else {
                self.bet *= 2;
            }
            None
        }
    }

    /// Perform the corresponding operations to the player's surrender if applicable,
    /// return an error message otherwise
    pub fn surrender(&mut self) -> Option<&str> {
        if self.hands.0.cards.len() != 2 {
            Some("Cannot surrender because you have already hit!")
        } else if self.hands.1.is_some() {
            Some("Cannot surrender because you have already splitted!")
        } else {
            self.bet /= 2;
            self.hands.0.points = 0;
            None
        }
    }

    /// Splits the player's hand if applicable, return an error message otherwise
    pub fn split(&mut self, deck: &mut Deck) -> Option<&str> {
        let first_hand_cards: &Vec<Card> = &self.hands.0.cards;

        if self.bet * 2 > self.actual_money {
            Some("Cannot split because you have not enough money!")
        } else if self.hands.1.is_some() {
            Some("Cannot split because you have already splitted!")
        } else if first_hand_cards.len() != 2 {
            Some("Cannot split because you have already hit!")
        } else if first_hand_cards[0].name != first_hand_cards[1].name {
            Some("Cannot split because your cards are not equal!")
        } else {
            self.bet *= 2;
            let cards: Vec<Card> = vec![
                self.hands
                    .1
                    .as_mut()
                    .unwrap()
                    .cards
                    .pop()
                    .expect("Failed to split"),
                deck.deal_card(),
            ];

            self.hands.1 = Some(Hand::from_cards(&cards));

            self.hands.0.deal_card(deck);
            None
        }
    }

    /// Perform the corresponding operations with the player's money,
    /// return the amount of money the player wins
    pub fn win(&mut self, hand_index: usize) -> u32 {
        let money_before = self.actual_money;
        if self.hands.1.is_none() {
            self.actual_money += self.bet;
            if self.hands.0.has_blackjack() {
                self.actual_money += self.bet / 2;
            }
        } else {
            // If the player has splitted, the money earned
            // by each winning hand should be the half,
            // since when player splitted, bet got doubled,
            // and each half of the bet represents one hand
            self.actual_money += self.bet / 2;
            if hand_index == 0 {
                if self.hands.0.has_blackjack() {
                    self.actual_money += self.bet / 4;
                }
            } else {
                if self
                    .hands
                    .1
                    .as_ref()
                    .map_or(false, |player| player.has_blackjack())
                {
                    self.actual_money += self.bet / 4;
                }
            }
        }
        self.actual_money - money_before
    }
    pub fn lose(&mut self) {
        if self.hands.1.is_none() {
            self.actual_money -= self.bet;
        } else {
            self.actual_money -= self.bet / 2;
        }
    }
}
