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
                continue;
            }
        }
    };
    let mut deck = Deck::new(n_of_decks);
    let mut dealer_hand = Hand::new(&mut deck);

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

fn start_game(players: &mut Vec<Player>, deck: &mut Deck) {
    let number_of_people: u8 = ask_number_of_people();
    ask_and_set_player_attributes(number_of_people, players, deck);
}

fn ask_number_of_people() -> u8 {
    loop {
        let number_of_people: u8 = match ask_user("\nHow many people are going to play? (1-7)")
            .trim()
            .parse()
        {
            Ok(val) => val,
            Err(_) => {
                println!("Expected integer input");
                continue;
            }
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
            let initial_money: i32 =
                match ask_user("How much money do you have? (Use only integer values)")
                    .trim()
                    .parse()
                {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Expected integer input");
                        continue;
                    }
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
        let bet: i32 = match ask_user("What bet do you wanna make? (Use only integral values)")
            .trim()
            .parse()
        {
            Ok(val) => val,
            Err(_) => {
                println!("Expected integer input");
                continue;
            }
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

fn hand_win_or_lose(hand: Hand) -> bool {
    let hand_points = hand.points;

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
    let player_points = Hand::calculate_points(&player.first_hand().cards);
    ask_player_bet(player);
    println!(
        "Your cards are:\n{} and {} ({} points)\n",
        player.first_hand().cards[0],
        player.first_hand().cards[1],
        // If the initial cards of the player are 2 aces, display correctly the points
        if player_points != 22 {
            player_points
        } else {
            12
        }
    );
    let mut has_splitted = false;
    for i in 0..2 {
        let mut hand = if i == 0 {
            player.hands.0.clone()
        } else {
            player.hands.1.clone().unwrap()
        };
        while !hand_win_or_lose(hand.clone()) {
            if has_splitted {
                println!("(Hand #{})", i + 1);
                println!("Your cards are: {}", hand.clone());
            }
            match ask_user("What do you want to do?\nAvailable Commands: (h)it, (s)tand, (sp)lit, (d)ouble, (surr)ender")
                .to_lowercase()
                .trim()
            {
                "hit" | "h" => {
                    player.hit(deck, i);
                    println!(
                        "Now, the cards of your {} hand are: {}",
                        if i == 0 { "first" } else { "second" },
                        if i == 0 {
                            player.hands.0.clone()
                        } else {
                            player.hands.1.clone().unwrap()
                        }
                    );
                }
                "stand" | "s" => {
                    println!("{} stood", player);
                    break;
                }
                "split" | "sp" => {
                    has_splitted = player.split(deck);
                    if has_splitted {
                        println!("You have splitted your hand!");
                    } else {
                        println!("You cannot split this hand!");
                    }
                }
                "double" | "d" => {
                    if player.double() {
                        println!("Bet doubled!");
                    } else {
                        println!("You cannot double your bet!");
                    }
                }
                "surrender" | "surr" => {
                    if player.surrender() {
                        println!("You surrendered!");
                        break;
                    } else {
                        println!("You cannot surrender now!");
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
        if player.hands.1.is_none() {
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
            if player_points == 21 || player_points > dealer_points {
                println!(
                    "{player} (#{hand_index} hand) won {money} :)\n",
                    player = player,
                    hand_index = i + 1,
                    money = player.actual_bet * 2
                );
                player.win();
            } else if player_points == 0 || player_points < dealer_points {
                println!(
                    "{}, your #{} hand lost against the dealer :(\n",
                    player,
                    i + 1
                );
                player.lose();
            } else {
                println!("It's a tie! :|");
            }
        }
    }
}

fn ask_if_next_game(player: &Player) -> bool {
    let mut player_next_game = false;
    let mut final_balance: String = format!("{} €", player.actual_money - player.initial_money);
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
        player.hand_mut().initialize_attributes(deck);
    }

    println!("\n\n\n");

    if !players.is_empty() {
        dealer_hand.initialize_attributes(deck);
        return true;
    }

    false
}
