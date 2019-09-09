mod deck;
mod hand;
mod player;

use std::io;
use std::io::Write;

use deck::{Card, Deck};
use hand::Hand;
use player::Player;

fn main() {
    println!("Welcome to BlackJack!\n");

    let mut players: Vec<Player> = Vec::new();
    let n_of_decks: u8 = loop {
        match ask_user("How many decks do you wanna use? (4-8)")
            .trim()
            .parse()
        {
            Ok(val) => {
                if val >= 4 && val <= 8 {
                    break val;
                } else {
                    println!("The number of decks must be between 4 and 8");
                }
            }
            Err(_) => {
                println!("Expected integer input");
            }
        }
    };
    let mut deck = Deck::new(n_of_decks);
    let mut dealer_hand = Hand::new(&mut deck);
    println!("####### Game Started #######");
    start_game(&mut players, &mut deck);
    loop {
        println!(
            "\nThe first card of the dealer is {}\n",
            dealer_hand.cards[0]
        );

        for player in players.iter_mut() {
            player_turn(player, &mut deck);
        }

        dealer_turn(&mut dealer_hand, &mut deck);
        end_game(&mut players, &dealer_hand);
        if !next_game(&mut players, &mut dealer_hand, &mut deck) {
            break;
        }
    }
}

fn ask_user(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}\n> ", prompt);

    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut input).expect("Failed to read");
    input
}

fn ask_user_number(prompt: &str) -> Option<u32> {
    return match ask_user(prompt).trim().parse() {
        Ok(val) => Some(val),
        Err(_) => {
            println!("Expected integer input");
            None
        }
    };
}

fn start_game(players: &mut Vec<Player>, deck: &mut Deck) {
    let number_of_people: u8 = ask_number_of_people();
    ask_and_set_player_attributes(number_of_people, players, deck);
}

fn ask_number_of_people() -> u8 {
    loop {
        let number_of_people: u8 =
            match ask_user_number("\nHow many people are going to play? (1-7)") {
                Some(value) => value as u8,
                None => continue,
            };

        if !(0 < number_of_people && number_of_people <= 7) {
            println!("The number of people must be between 1 and 7");
        } else {
            break number_of_people;
        }
    }
}

fn ask_and_set_player_attributes(number_of_people: u8, players: &mut Vec<Player>, deck: &mut Deck) {
    for i in 0..number_of_people {
        let name: String = ask_user(format!("\nPlease, enter your name player {}", i + 1).as_str());
        loop {
            let initial_money: u32 =
                match ask_user_number("How much money do you have? (Use only integer values)") {
                    Some(value) => value,
                    None => continue,
                };

            if initial_money < 50 {
                println!("The initial money must be greater or equal than 50\n");
            } else {
                players.push(Player::new(name.trim().to_string(), initial_money, deck));
                break;
            }
        }
    }
}

fn ask_player_bet(player: &mut Player) {
    loop {
        let bet: u32 =
            match ask_user_number("What bet do you wanna make? (Use only integral values)") {
                Some(value) => value,
                None => continue,
            };

        if bet > player.actual_money {
            println!("Your bet cannot be greater than your actual money.\n");
        } else if bet <= 0 {
            println!("Your bet must be greater than 0.\n");
        } else {
            player.bet(bet);
            break;
        }
    }
}

fn hand_win_or_lose(hand: &Hand) -> bool {
    if hand.has_blackjack() {
        println!("BLACKJACK!");
        return true;
    } else {
        let hand_points: u8 = hand.points;
        match hand_points {
            21 => {
                println!("YOU GOT 21 POINTS!");
                return true;
            }
            0 => {
                println!("BUST.\nI'm afraid you lose this game :(\n");
                return true;
            }
            _ => (),
        }
    }
    false
}

fn check_if_yes(user_decision: &str) -> bool {
    ["y", "yes", "1", "true"].contains(&user_decision.to_lowercase().trim())
}

fn player_turn(player: &mut Player, deck: &mut Deck) {
    println!(
        "\n{player}, your actual money is {actual_money} €\n",
        player = player,
        actual_money = player.actual_money
    );
    let player_first_hand_cards: Vec<Card> = player.hands.0.cards.clone();

    ask_player_bet(player);
    println!(
        "\nYour cards are:\n{} and {} ({} points)\n",
        player_first_hand_cards[0], player_first_hand_cards[1], player.hands.0.points
    );
    let mut has_doubled = false;
    let mut has_splitted = false;
    let mut hit_counter: u8 = 0;
    for i in 0..2 {
        let mut hand = if i == 0 {
            player.hands.0.clone()
        } else {
            player.hands.1.clone().unwrap()
        };
        // If the player has doubled, he can only ask for one more card
        while !hand_win_or_lose(&hand) && (!has_doubled || hit_counter < 1) {
            if has_splitted {
                println!("(Hand #{})", i + 1);
                println!("Your cards are: {}", hand);
            }
            match ask_user("What do you want to do?\nAvailable Commands: (h)it, (s)tand, (sp)lit, (d)ouble, (surr)ender")
                .to_lowercase()
                .trim()
            {
                "h" | "hit" => {
                    player.hit(deck, i);
                    println!(
                        "Now, the cards of your {} hand are: {}",
                        if i == 0 { "first" } else { "second" },
                        // Update the hand
                        if i == 0 {
                            player.hands.0.clone()
                        } else {
                            player.hands.1.clone().unwrap()
                        }
                    );
                    hit_counter += 1;
                }
                "s" | "stand" => {
                    println!("{} stood", player);
                    break;
                }
                "sp" | "split" => {
                    if !has_doubled {
                        match player.split(deck) {
                            Ok(()) => {
                                has_splitted = true;
                                println!("You have splitted the hand!\n")
                            }
                            Err(msg) => println!("{}", msg)
                        }
                    } else {
                        println!("Cannot split because you have already doubled\n");
                    }
                }
                "d" | "double" => {
                    // Checks if the player had already doubled
                    if !has_doubled {
                        match player.double() {
                            Ok(()) => {
                                has_doubled = true;
                                println!("You have doubled your hand!\n")
                            }
                            Err(msg) => println!("{}", msg)
                        }
                    } else {
                        println!("Cannot double more than once!\n");
                    }
                }
                "surr" | "surrender" => {
                    if !has_doubled {
                        match player.surrender() {
                            Ok(()) => println!("You have surrendered!\n"),
                            Err(msg) => println!("{}", msg)
                        }
                    } else {
                        println!("Cannot surrender because you have already doubled\n");
                    }
                }

                _ => println!("Invalid command!\nAvailable Commands: (h)it, (s)tand, (sp)lit, (d)ouble, (surr)ender"),
            }
            // Update the hand
            hand = if i == 0 {
                player.hands.0.clone()
            } else {
                player.hands.1.clone().unwrap()
            };
        }
        if !has_splitted {
            break;
        }
    }
}

fn dealer_lost(dealer_hand: &Hand) -> bool {
    if dealer_hand.points == 0 {
        println!("The dealer busted. The game ended :)\n");
        return true;
    }
    false
}

fn dealer_turn(dealer_hand: &mut Hand, deck: &mut Deck) {
    println!(
        "\nThe dealer's cards are {} and {}\n",
        dealer_hand.cards[0], dealer_hand.cards[1]
    );
    while !dealer_lost(dealer_hand) && dealer_hand.points < 17 {
        println!("The dealer is going to hit a card\n");
        dealer_hand.deal_card(deck);
        println!("Now, the cards of the dealer are: {}", dealer_hand);
    }
}

fn end_game(players: &mut Vec<Player>, dealer_hand: &Hand) {
    println!("####### Game Finished #######\n");
    let dealer_points = dealer_hand.points;

    for player in players.iter_mut() {
        for i in 0..2 {
            let hand_splitted = player.hands.1.clone();
            if i == 1 && hand_splitted.is_none() {
                break;
            }
            let player_points = if i == 0 {
                player.hands.0.points
            } else {
                hand_splitted.unwrap().points
            };
            if player_points > dealer_points && !dealer_hand.has_blackjack() {
                let money_earned: u32 = player.win();
                println!(
                    "{player} (#{hand_index} hand) won {money}€! :)\n",
                    player = player,
                    hand_index = i + 1,
                    money = money_earned
                );
            } else if player_points == 0 || player_points < dealer_points {
                println!(
                    "{player} (#{hand_index} hand) lost! :(\n",
                    player = player,
                    hand_index = i + 1
                );
                player.lose();
            } else {
                println!(
                    "{player} (#{hand_index} hand) tied! :|\n",
                    player = player,
                    hand_index = i + 1
                );
            }
        }
    }
}

fn ask_if_next_game(player: &Player) -> bool {
    let mut player_next_game = false;
    // Since unsigned ints cannot be negative, I need to cast the values to i64 to avoid errors.
    // Casting to a i32 int would cause errors if the values exceeded the i32 limit.
    let mut final_balance: String = format!(
        "{} €",
        player.actual_money as i64 - player.initial_money as i64
    );
    if !final_balance.starts_with("-") {
        final_balance.insert(0, '+');
    }
    if player.actual_money > 0 {
        let decision =
            ask_user(format!("\n{}, do you want to play again? (y/n)\n", player).as_str());

        if check_if_yes(decision.as_str()) {
            player_next_game = true;
        } else {
            println!(
                "Thanks for playing, {player}, your final balance is {final_balance}\n",
                player = player,
                final_balance = final_balance
            );
        }
    } else {
        println!(
            "{}, you have lost all your money. Thanks for playing\n",
            player
        );
    }
    player_next_game
}

fn next_game(players: &mut Vec<Player>, dealer_hand: &mut Hand, deck: &mut Deck) -> bool {
    players.retain(|player| ask_if_next_game(player));

    for player in players.iter_mut() {
        player.reset_hands(deck);
    }

    println!("\n\n\n");

    if !players.is_empty() {
        dealer_hand.initialize_attributes(deck);
        return true;
    }

    false
}
