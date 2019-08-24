mod deck;
mod hand;
mod player;

use std::io;
use std::io::Write;

use deck::{Card, Deck};
use hand::Hand;
use player::Player;

fn main() {
    let mut players: Vec<Player> = Vec::new();
    let mut deck = Deck::new();
    let mut dealer_hand = Hand::new(&mut deck);

    start_game(&mut players, &mut deck);
    loop {
        println!("Welcome to BlackJack!\n");

        println!("The first card of the dealer is {}", dealer_hand.cards[0]);

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
        let number_of_people: u8 = match ask_user("How many people are going to play? (1-7)")
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
        let name: String = ask_user(format!("Please, enter your name player {}", i + 1).as_str());
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

fn player_win_or_lose(player: &Player) -> bool {
    let mut result = false;
    let player_points = player.hand.points;

    match player_points {
        21 => {
            println!("BLACKJACK");
            result = true;
        }
        0 => {
            println!("BUST.\n\t\t\t\tI'm afraid you lose this game :(\n");
            result = true;
        }
        _ => (),
    }
    result
}

fn check_if_yes(user_decision: &str) -> bool {
    ["y", "yes", "1", "true"].contains(&user_decision.trim().to_lowercase().as_str())
}

fn ask_if_hit() -> bool {
    let decision = ask_user("Do you wanna hit? (y/n)\n");
    check_if_yes(decision.as_str())
}

fn player_turn(player: &mut Player, deck: &mut Deck) {
    println!(
        "{player}, your actual money is {actual_money} €\n",
        player = player,
        actual_money = player.actual_money
    );

    ask_player_bet(player);

    println!(
        "Your cards are:\n{} and {}\n",
        player.hand.cards[0], player.hand.cards[1]
    );

    while !player_win_or_lose(player) {
        if ask_if_hit() {
            player.hand.deal_card(deck);
            println!("Now, your cards are: {}", player.hand);
        } else {
            println!("{} stood", player);
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
        "The dealer's cards are {} and {}\n",
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
        let player_points = player.hand.points;

        if player_points == 21 || player_points > dealer_points {
            println!(
                "{player} won {money} :)\n",
                player = player,
                money = player.actual_bet * 2
            );
            player.win();
        } else if player_points == 0 || player_points < dealer_points {
            println!("{} lost against the dealer :(\n", player);
            player.lose();
        } else {
            println!("It's a tie! :|");
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
        let decision = ask_user(format!("{}, do you want to play again? (y/n)\n", player).as_str());

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
        player.hand.initialize_attributes(deck);
    }

    println!("\n\n\n");

    if !players.is_empty() {
        dealer_hand.initialize_attributes(deck);
        return true;
    }

    false
}
