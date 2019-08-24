use crate::{Deck, Hand};
use std::fmt;

pub struct Player {
    pub hand: Hand,
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
            hand: Hand::new(deck),
            name,
            initial_money,
            actual_money: initial_money,
            actual_bet: 0,
        }
    }

    pub fn bet(&mut self, money: i32) {
        self.actual_bet = money;
    }
    pub fn win(&mut self) {
        self.actual_money += self.actual_bet;
    }
    pub fn lose(&mut self) {
        self.actual_money -= self.actual_bet;
    }
}
