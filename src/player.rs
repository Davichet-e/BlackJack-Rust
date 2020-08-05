use crate::deck::{Card, Deck};
use crate::hand::Hand;

use std::fmt;

#[derive(Clone)]
pub struct HandWrap {
    hand: Hand,
    splitted_hand: Option<Hand>,
}

#[derive(Clone)]
pub struct Player {
    hand: HandWrap,
    name: String,
    initial_money: u32,
    actual_money: u32,
    bet: u32,
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
            hand: HandWrap {
                hand: Hand::new(deck),
                splitted_hand: None,
            },
            name,
            initial_money,
            actual_money: initial_money,
            bet: 0,
        }
    }
    pub fn hand(&self) -> &HandWrap {
        &self.hand
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn initial_money(&self) -> u32 {
        self.initial_money
    }
    pub fn actual_money(&self) -> u32 {
        self.actual_money
    }
    pub fn bet(&self) -> u32 {
        self.bet
    }

    pub fn hand_mut(&mut self) -> &mut HandWrap {
        &mut self.hand
    }
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    pub fn initial_money_mut(&mut self) -> &mut u32 {
        &mut self.initial_money
    }
    pub fn actual_money_mut(&mut self) -> &mut u32 {
        &mut self.actual_money
    }
    pub fn bet_mut(&mut self) -> &mut u32 {
        &mut self.bet
    }
    /// Reset the attributes of the `Hand`s of the instance.
    pub fn reset_hands(&mut self, deck: &mut Deck) {
        self.hand.hand.initialize_attributes(deck);
        if let Some(hand) = self.hand.splitted_hand {
            hand.initialize_attributes(deck);
        }
    }

    pub fn make_bet(&mut self, money: u32) {
        self.bet = money;
    }

    pub fn hit(&mut self, deck: &mut Deck, splitted_hand: bool) {
        if splitted_hand {
            self.hand.splitted_hand.unwrap().deal_card(deck);
        } else {
            self.hand.hand.deal_card(deck);
        }
    }

    /// Double the player's bet if applicable, return an error message otherwise
    pub fn double(&mut self) -> Option<&str> {
        if self.bet * 2 > self.actual_money {
            Some("Cannot double because you have not enough money!")
        } else if self.hand.hand.cards.len() > 2 {
            Some("Cannot double because you have already hit!")
        } else {
            if self.hand.splitted_hand.is_some() {
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
        if self.hand.hand.cards.len() != 2 {
            Some("Cannot surrender because you have already hit!")
        } else if self.hand.splitted_hand.is_some() {
            Some("Cannot surrender because you have already splitted!")
        } else {
            self.bet /= 2;
            self.hand.hand.points = 0;
            None
        }
    }

    /// Splits the player's hand if applicable, return an error message otherwise
    pub fn split(&mut self, deck: &mut Deck) -> Option<&str> {
        let first_hand_cards: &Vec<Card> = &self.hand.hand.cards;

        if self.bet * 2 > self.actual_money {
            Some("Cannot split because you have not enough money!")
        } else if self.hand.splitted_hand.is_some() {
            Some("Cannot split because you have already splitted!")
        } else if first_hand_cards.len() != 2 {
            Some("Cannot split because you have already hit!")
        } else if first_hand_cards[0].name != first_hand_cards[1].name {
            Some("Cannot split because your cards are not equal!")
        } else {
            self.bet *= 2;
            let cards: Vec<Card> = vec![
                self.hand.hand.cards.pop().expect("Failed to split"),
                deck.deal_card(),
            ];

            self.hand.splitted_hand = Some(Hand::from_cards(&cards));

            self.hand.hand.deal_card(deck);
            None
        }
    }

    /// Perform the corresponding operations with the player's money,
    /// return the amount of money the player wins
    pub fn win(&mut self, splitted_hand: bool) -> u32 {
        let money_before = self.actual_money;
        if self.hand.splitted_hand.is_none() {
            self.actual_money += self.bet;
            if !splitted_hand && self.hand.hand.has_blackjack()
                || splitted_hand
                    && self
                        .hand
                        .splitted_hand
                        .map_or(false, |hand| hand.has_blackjack())
            {
                self.actual_money += self.bet / 2;
            }
        } else {
            // If the player has splitted, the money earned
            // by each winning hand should be the half,
            // since when player splitted, bet got doubled,
            // and each half of the bet represents one hand
            self.actual_money += self.bet / 2;
            if self.hands[hand_index].has_blackjack() {
                self.actual_money += self.bet / 4;
            }
        }
        self.actual_money - money_before
    }
    pub fn lose(&mut self) {
        if self.hands.len() == 1 {
            self.actual_money -= self.bet;
        } else {
            self.actual_money -= self.bet / 2;
        }
    }
}
