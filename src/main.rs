use std::io;

use mcts::connect_four::Player::{Player1, Player2};
use mcts::connect_four::*;

use crate::GameResult::{Draw, Win};

fn main() {
    let game_result = play_game();
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
fn play_game() -> GameResult {
    print!("{}", ansi_escapes::ClearScreen);

    let mut current_player = Player1;
    let mut gb = GameBoard::default();

    loop {
        println!("{}", gb);
        if gb.check_draw() {
            return Draw;
        }
        println!("{}, please select a column", current_player);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input!");

        match input.trim().parse() {
            Ok(position) => match gb.drop_stone(current_player, position) {
                // Invalid move by player
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
                // A valid move, but no victory
                Some(false) => {
                    print!(
                        "{}",
                        ansi_escapes::EraseLines((gb.get_dimensions().0 + 6) as u16)
                    );
                    match current_player {
                        Player1 => current_player = Player2,
                        Player2 => current_player = Player1,
                    }
                    continue;
                }
                // Victory by the current player
                Some(true) => {
                    print!(
                        "{}{}",
                        ansi_escapes::EraseLines((gb.get_dimensions().0 + 6) as u16),
                        gb
                    );
                    return Win(current_player);
                }
            },
            Err(_) => continue,
        };
    }
}

enum GameResult {
    Draw,
    Win(Player),
}
