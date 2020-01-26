use crate::deck::{Card, Deck};
use crate::hand::Hand;

use std::fmt;

#[derive(Clone)]
pub struct Player {
    pub hands: Vec<Hand>,
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
            hands: vec![Hand::new(deck)],
            name,
            initial_money,
            actual_money: initial_money,
            bet: 0,
        }
    }
    /// Reset the attributes of the `Hand`s of the instance.
    pub fn reset_hands(&mut self, deck: &mut Deck) {
        self.hands
            .iter_mut()
            .for_each(|hand| hand.initialize_attributes(deck))
    }

    pub fn bet(&mut self, money: u32) {
        self.bet = money;
    }

    pub fn hit(&mut self, deck: &mut Deck, hand_index: usize) {
        self.hands[hand_index].deal_card(deck)
    }

    /// Double the player's bet if applicable, return an error message otherwise
    pub fn double(&mut self) -> Option<&str> {
        if self.bet * 2 > self.actual_money {
            Some("Cannot double because you have not enough money!")
        } else if self.hands[0].cards.len() > 2 {
            Some("Cannot double because you have already hit!")
        } else {
            if self.hands.get(1).is_some() {
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
        if self.hands[0].cards.len() != 2 {
            Some("Cannot surrender because you have already hit!")
        } else if self.hands.get(1).is_some() {
            Some("Cannot surrender because you have already splitted!")
        } else {
            self.bet /= 2;
            self.hands[0].points = 0;
            None
        }
    }

    /// Splits the player's hand if applicable, return an error message otherwise
    pub fn split(&mut self, deck: &mut Deck) -> Option<&str> {
        let first_hand_cards: &Vec<Card> = &self.hands[0].cards;

        if self.bet * 2 > self.actual_money {
            Some("Cannot split because you have not enough money!")
        } else if self.hands.get(1).is_some() {
            Some("Cannot split because you have already splitted!")
        } else if first_hand_cards.len() != 2 {
            Some("Cannot split because you have already hit!")
        } else if first_hand_cards[0].name != first_hand_cards[1].name {
            Some("Cannot split because your cards are not equal!")
        } else {
            self.bet *= 2;
            let cards: Vec<Card> = vec![
                self.hands[0].cards.pop().expect("Failed to split"),
                deck.deal_card(),
            ];

            self.hands.push(Hand::from_cards(&cards));

            self.hands[0].deal_card(deck);
            None
        }
    }

    /// Perform the corresponding operations with the player's money,
    /// return the amount of money the player wins
    pub fn win(&mut self) -> u32 {
        let money_before: u32 = self.actual_money;
        self.actual_money += self.bet;

        // If has a BlackJack, sums 1.5 times the actual bet, otherwise just 1 time
        if self.hands[0].has_blackjack() {
            self.actual_money += self.bet / 2;
        }
        if self.hands.get(1).map_or(false, |hand| hand.has_blackjack()) {
            self.actual_money += self.bet / 2;
        }
        self.actual_money - money_before
    }
    pub fn lose(&mut self) {
        self.actual_money -= self.bet;
    }
}
