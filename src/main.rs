use std::io;

use mcts::connect_four::Owner::{Player1, Player2};
use mcts::connect_four::*;

fn main() {
    print!("{}", ansi_escapes::ClearScreen);

    let mut player = Player1;
    let mut gb = GameBoard::default();

    loop {
        print!("{}", gb);
        println!("{}, please select a column", player);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input!");

        match input.trim().parse() {
            Ok(position) => match gb.drop_stone(player, position) {
                None => {
                    print!(
                        "{}",
                        ansi_escapes::EraseLines((gb.get_dimensions().0 + 5) as u16)
                    );
                    println!(
                        "Invalid move ({}) by {}! - The other player wins!",
                        position, player
                    );
                    match player {
                        Player1 => player = Player2,
                        Player2 => player = Player1,
                    };
                    break;
                }
                Some(false) => match player {
                    Player1 => player = Player2,
                    Player2 => player = Player1,
                },
                Some(true) => {
                    print!(
                        "{}",
                        ansi_escapes::EraseLines((gb.get_dimensions().0 + 5) as u16)
                    );
                    break;
                }
            },
            Err(_) => {}
        };
        print!(
            "{}",
            ansi_escapes::EraseLines((gb.get_dimensions().0 + 5) as u16)
        );
    }

    print!("{}", gb);
    println!("{} wins!", player);
    println!("Excellent!");
}
