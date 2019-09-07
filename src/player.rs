use crate::{Card, Deck, Hand};
use std::fmt;

pub struct Player {
    pub hands: (Hand, Option<Hand>),
    pub name: String,
    pub initial_money: u32,
    pub actual_money: u32,
    pub actual_bet: u32,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Player {
    pub fn new(name: String, initial_money: u32, deck: &mut Deck) -> Player {
        Player {
            hands: (Hand::new(deck), None),
            name,
            initial_money,
            actual_money: initial_money,
            actual_bet: 0,
        }
    }

    pub fn reset_hands(&mut self, deck: &mut Deck) {
        self.hands.0.initialize_attributes(deck);

        if let Some(hand) = &mut self.hands.1 {
            hand.initialize_attributes(deck);
        }
    }

    pub fn bet(&mut self, money: u32) {
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

    pub fn double(&mut self) -> Result<(), &str> {
        if self.actual_bet * 2 > self.actual_money {
            return Err("Cannot double because you have not enough money!");
        } else if self.hands.0.cards.len() != 2 {
            return Err("Cannot double because you have already hit!");
        } else if self.hands.1.is_some() {
            return Err("Cannot double because you have already splitted!");
        }

        self.actual_bet *= 2;
        Ok(())
    }

    pub fn surrender(&mut self) -> Result<(), &str> {
        if self.hands.0.cards.len() != 2 {
            return Err("Cannot surrender because you have already hit!");
        } else if self.hands.1.is_some() {
            return Err("Cannot surrender because you have already splitted!");
        }
        self.actual_bet /= 2;
        self.hands.0.points = 0;
        Ok(())
    }

    pub fn split(&mut self, deck: &mut Deck) -> Result<(), &str> {
        let ref mut first_hand_cards: Vec<Card> = self.hands.0.cards;
        if self.actual_bet * 2 > self.actual_money {
            return Err("Cannot split because you have not enough money!");
        } else if self.hands.1.is_some() {
            return Err("Cannot split because you have already splitted!");
        } else if first_hand_cards.len() != 2 {
            return Err("Cannot split because you have already hit!");
        } else if first_hand_cards[0].name != first_hand_cards[1].name {
            return Err("Cannot split because your cards are not equal!");
        }

        self.actual_bet *= 2;

        let cards: Vec<Card> = vec![
            first_hand_cards.pop().expect("Failed to split"),
            deck.deal_card(),
        ];
        let points: u8 = Hand::calculate_points(&cards);
        self.hands.1 = Some(Hand {
            cards,
            points,
            aces: 0,
        });

        self.hands.0.deal_card(deck);
        Ok(())
    }
    pub fn win(&mut self) -> u32 {
        let money_before: u32 = self.actual_money;
        self.actual_money += self.actual_bet;

        // If has a BlackJack, sums 1.5 times the actual bet, otherwise just 1 time
        if self.hands.0.has_blackjack() {
            self.actual_money += self.actual_bet / 2;
        }
        if self.hands.1.is_some() && self.hands.1.as_ref().unwrap().has_blackjack() {
            self.actual_money += self.actual_bet / 2;
        }
        self.actual_money - money_before
    }
    pub fn lose(&mut self) {
        self.actual_money -= self.actual_bet;
    }
}
