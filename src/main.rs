use std::io;

use mcts::connect_four::GameResult::{Draw, Win};
use mcts::connect_four::Player::{Player1, Player2};
use mcts::connect_four::*;
use mcts::mcts::Node;

fn main() {
    prepare_game();
}

fn prepare_game() {
    println!("0 for a 2 player game - 1 or 2 for your position against the ai");
    let mut input = String::new();
    let game_result;

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input!");

        match input.trim() {
            "0" => {
                game_result = play_game(None);
                break;
            }
            "1" => {
                game_result = play_game(Some(Player1));
                break;
            }
            "2" => {
                game_result = play_game(Some(Player2));
                break;
            }
            _ => continue,
        }
    }

    println!();
    match game_result {
        Draw => {
            println!("Draw!")
        }
        Win(player) => {
            println!("{} wins!", player);
            println!("Excellent!");
        }
    }
}

/// Plays until:
/// A: A player has won
/// B: An invalid move is made by a player (the opponent wins in this case)
/// C: A draw is reached (the board is full)
/// Returns the GameResult.
fn play_game(player_position: Option<Player>) -> mcts::connect_four::GameResult {
    let mut current_player = Player1;
    let mut game_state = Node::new(GameBoard::default(), Player1);

    while let GameState::Ongoing = game_state.get_board().get_game_state() {
        print!("{}", ansi_escapes::ClearScreen);
        println!("{}", game_state.get_board());

        if player_position == None || player_position == Some(current_player) {
            println!("Input your move:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read user input!");

            match input.trim().parse() {
                Ok(position) => match game_state.make_move(GameMove::new(position)) {
                    None => {
                        println!(
                            "Invalid move ({}) by {}! - The other player wins!",
                            position, current_player
                        );
                        return match current_player {
                            Player1 => Win(Player2),
                            Player2 => Win(Player1),
                        };
                    }
                    Some(n) => {
                        game_state = n;
                    }
                },
                Err(_) => continue,
            }
        } else {
            for _ in 0..10000000 {
                game_state.mcts();
            }
            let best_move = game_state.best_move();
            match game_state.make_move(best_move) {
                Some(n) => {
                    game_state = n;
                }
                None => {
                    unreachable!();
                }
            }
        }
        match current_player {
            Player1 => current_player = Player2,
            Player2 => current_player = Player1,
        }
    }
    print!("{}", ansi_escapes::ClearScreen);
    println!("{}", game_state.get_board());

    return match game_state.get_board().get_game_state() {
        GameState::GameResult(result) => result,
        GameState::Ongoing => {
            unreachable!()
        }
    };
}
