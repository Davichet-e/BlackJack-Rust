mod deck;
mod hand;
mod player;

use std::io;
use std::io::Write;

use deck::Deck;
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
    match ask_user(prompt).trim().parse::<i64>() {
        Ok(val) if val > 0 => Some(val as u32),
        Ok(_) => {
            println!("The number must be greater than 0.\n");
            None
        }
        Err(_) => {
            println!("Expected integer input.");
            None
        }
    }
}

fn start_game(players: &mut Vec<Player>, deck: &mut Deck) {
    let number_of_people: u8 = ask_number_of_people();
    ask_and_set_player_attributes(number_of_people, players, deck);
}

fn ask_number_of_people() -> u8 {
    loop {
        let number_of_people: u32 =
            match ask_user_number("\nHow many people are going to play? (1-7)") {
                Some(value) => value,
                None => continue,
            };

        if !(0 < number_of_people && number_of_people <= 7) {
            println!("The number of people must be between 1 and 7");
        } else {
            break number_of_people as u8;
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
                players.push(Player::new(String::from(name.trim()), initial_money, deck));
                break;
            }
        }
    }
}

fn ask_player_bet(player: &Player) -> u32 {
    loop {
        let bet: u32 =
            match ask_user_number("What bet do you wanna make? (Use only integral values)") {
                Some(value) => value,
                None => continue,
            };

        if bet > player.actual_money {
            println!("Your bet cannot be greater than your actual money.\n");
        } else {
            break bet;
        }
    }
}

fn hand_win_or_lose(hand: &Hand) -> bool {
    if hand.has_blackjack() {
        println!("BLACKJACK!\n");
        true
    } else {
        match hand.points {
            21 => {
                println!("YOU GOT 21 POINTS!\n");
                true
            }
            0 => {
                println!("BUST.\nI'm afraid you lose this game :(\n");
                true
            }
            _ => false,
        }
    }
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
    let bet: u32 = ask_player_bet(player);
    player.bet(bet);
    let player_first_hand: &Hand = &player.hands[0];
    println!(
        "\nYour cards are:\n{} and {} ({} points)\n",
        player_first_hand.cards[0], player_first_hand.cards[1], player_first_hand.points
    );
    for i in 0..2 {
        let mut has_doubled = false;
        while !hand_win_or_lose(&player.hands[i])
            // If the player has doubled, he can only ask for one more card
            && (!has_doubled || player.hands[i].cards.len() < 3)
        {
            if player.hands.len() > 1 {
                println!("\n(Hand #{})", i + 1);
            }
            match ask_user("What do you want to do?\nAvailable Commands: (h)it, (s)tand, (sp)lit, (d)ouble, (surr)ender")
                .to_lowercase()
                .trim()
            {
                "h" | "hit" => {
                    player.hit(deck, i);
                    println!(
                        "Now, the cards are: {}",
                        player.hands[i]
                    );
                }
                "s" | "stand" => {
                    println!("{} stood", player);
                    break;
                }
                "sp" | "split" => {
                    if !has_doubled {
                        match player.split(deck) {
                            Some(error_message) => println!("{}", error_message),
                            None => {
                                println!("You have splitted the hand!\n")
                            }
                        }
                    } else {
                        println!("Cannot split because you have already doubled\n");
                    }
                }
                "d" | "double" => {
                    if !has_doubled {
                        match player.double() {
                            Some(error_message) => println!("{}", error_message),
                            None => {
                                has_doubled = true;
                                println!("You have doubled your hand!\n")
                            }
                        }
                    } else {
                        println!("Cannot double more than once!\n");
                    }
                }
                "surr" | "surrender" => {
                    if !has_doubled {
                        match player.surrender() {
                            Some(error_message) => println!("{}", error_message),
                            None => println!("You have surrendered!\n")
                        }
                    } else {
                        println!("Cannot surrender because you have already doubled\n");
                    }
                }

                _ => println!("Invalid command!\nAvailable Commands: (h)it, (s)tand, (sp)lit, (d)ouble, (surr)ender"),
            }
        }
        if player.hands.len() == 1 {
            break;
        }
    }
}

fn dealer_lost(dealer_hand: &Hand) -> bool {
    if dealer_hand.points == 0 {
        println!("The dealer busted. The game ended :)\n");
        true
    } else {
        false
    }
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

fn end_game(players: &mut [Player], dealer_hand: &Hand) {
    println!("####### Game Finished #######\n");
    let dealer_points = dealer_hand.points;

    for player in players.iter_mut() {
        for (i, hand) in player.hands.clone().iter().enumerate() {
            let hand_points: u8 = hand.points;
            if hand_points > dealer_points
                || player.hands[0].has_blackjack() && !dealer_hand.has_blackjack()
            {
                let money_earned: u32 = player.win();
                println!(
                    "{player}{} won {money}€! :)\n",
                    // If it hasn't splitted, don't show the hand's index
                    if player.hands.len() == 1 {
                        String::new()
                    } else {
                        format!(" (#{} hand)", i + 1)
                    },
                    player = player,
                    money = money_earned,
                );
            } else if hand_points == 0 || hand_points < dealer_points {
                println!(
                    "{player}{} lost! :(\n",
                    if player.hands.len() == 1 {
                        String::new()
                    } else {
                        format!(" (#{} hand)", i + 1)
                    },
                    player = player,
                );
                player.lose();
            } else {
                println!(
                    "{player}{} tied! :|\n",
                    if player.hands.len() == 1 {
                        String::new()
                    } else {
                        format!(" (#{} hand)", i + 1)
                    },
                    player = player,
                );
            }
        }
    }
}

fn ask_if_next_game(player: &Player) -> bool {
    let player_next_game: bool;

    let final_balance: String = format!(
        "{} €",
        if player.actual_money >= player.initial_money {
            format!("+{}", player.actual_money - player.initial_money)
        } else {
            // Since unsigned integers cannot be negative, I need to cast the values to i64 to avoid errors.
            // Casting to a i32 int would cause errors if the values exceeded the i32 limit.
            (i64::from(player.actual_money) - i64::from(player.initial_money)).to_string()
        }
    );

    if player.actual_money > 0 {
        let decision: String =
            ask_user(format!("\n{}, do you want to play again? (y/n)\n", player).as_str());

        if check_if_yes(decision.as_str()) {
            player_next_game = true;
        } else {
            player_next_game = false;
            println!(
                "Thanks for playing, {}, your final balance is {}\n",
                player, final_balance
            );
        }
    } else {
        player_next_game = false;
        println!(
            "{}, you have lost all your money. Thanks for playing\n",
            player
        );
    }
    player_next_game
}

fn next_game(players: &mut Vec<Player>, dealer_hand: &mut Hand, deck: &mut Deck) -> bool {
    players.retain(ask_if_next_game);

    for player in players.iter_mut() {
        player.reset_hands(deck);
    }

    println!("\n\n\n");

    if !players.is_empty() {
        dealer_hand.initialize_attributes(deck);
        true
    } else {
        false
    }
}
