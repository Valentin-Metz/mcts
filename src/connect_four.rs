use std::fmt;
use std::fmt::Formatter;

use colored::*;

use crate::connect_four::GameResult::Draw;
use crate::connect_four::GameState::Ongoing;
use crate::connect_four::Player::{Player1, Player2};
use rand::{thread_rng, Rng};

/// GameBoard - implemented as a 2D struct array.
/// A field can be empty (None) - or occupied by one of tho two players (Some(Player)).
/// Default size is 7 long (indexed by x); 6 high (indexed by y).
#[derive(Debug, Copy, Clone)]
pub struct GameBoard {
    board: [[Option<Player>; 7]; 6],
    game_state: GameState,
}

/// Pretty prints the GameBoard. Uses ANSI-Escape-Codes for color and in-place prints.
impl std::fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "| 0 | 1 | 2 | 3 | 4 | 5 | 6 |\n")?;
        for column in self.board.iter().rev() {
            write!(f, "|")?;
            for owner in column {
                match owner {
                    None => {
                        write!(f, "   |")?;
                    }
                    Some(Player::Player1) => {
                        write!(f, " {} |", "X".red())?;
                    }
                    Some(Player::Player2) => {
                        write!(f, " {} |", "O".blue())?;
                    }
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            board: [[None; 7]; 6],
            game_state: GameState::Ongoing,
        }
    }
    pub fn default() -> GameBoard {
        GameBoard::new()
    }

    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }

    pub fn get_possible_moves<'a>(&'a self) -> impl Iterator<Item = GameMove> + 'a {
        self.board
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.is_none())
            .map(|(m, _)| GameMove::new(m))
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (
            self.board.len(),
            self.board
                .get(0)
                .and_then(|column| Some(column.len()))
                .unwrap_or(0),
        )
    }

    /// Returns a single square of our GameBoard.
    pub fn get_field(&self, coordinate: Coordinate) -> Option<Player> {
        self.board[coordinate.y][coordinate.x]
    }

    /// Drops a stone into a specified column (from the top).
    /// If the coordinate is invalid, None will be returned.
    /// If the coordinate is valid, the returned value indicates:
    /// A: The current player has won after this move (Some(true)).
    /// B: The game goes on (Some(false)) - (this includes a potential draw).
    pub fn drop_stone(&mut self, player: Player, game_move: GameMove) {
        for i in 0..self.board.len() {
            if self.board[i][game_move.x].is_none() {
                self.board[i][game_move.x] = Some(player);
                if self.check_win(
                    player,
                    Coordinate {
                        y: i,
                        x: game_move.x,
                    },
                ) {
                    self.game_state = GameState::GameResult(GameResult::Win(player));
                } else if self.check_draw() {
                    self.game_state = GameState::GameResult(Draw)
                }
                return;
            }
        }
        unreachable!();
    }

    /// Checks if no further stones can be placed on the board.
    fn check_draw(&self) -> bool {
        self.board
            .last()
            .and_then(|row| Some(row.iter().filter(|cell| cell.is_some()).count() == row.len()))
            .unwrap_or(true)
    }

    /// Checks on four stones in sequence.
    /// Only the coordinates around the origin are evaluated.
    /// Directional movement is performed by stepping functions.
    pub fn check_win(&self, player: Player, coordinate: Coordinate) -> bool {
        let left_right = |c: Coordinate| Coordinate {
            y: c.y,
            x: c.x.wrapping_add(1),
        };
        let right_left = |c: Coordinate| Coordinate {
            y: c.y,
            x: c.x.wrapping_sub(1),
        };
        let bottom_left_top_right = |c: Coordinate| Coordinate {
            y: c.y.wrapping_add(1),
            x: c.x.wrapping_add(1),
        };
        let top_right_bottom_left = |c: Coordinate| Coordinate {
            y: c.y.wrapping_sub(1),
            x: c.x.wrapping_sub(1),
        };
        let top_left_bottom_right = |c: Coordinate| Coordinate {
            y: c.y.wrapping_sub(1),
            x: c.x.wrapping_add(1),
        };
        let bottom_right_top_left = |c: Coordinate| Coordinate {
            y: c.y.wrapping_add(1),
            x: c.x.wrapping_sub(1),
        };
        // Only needed in one direction (As no stone can be above a newly inserted one)
        let top_to_bottom = |c: Coordinate| Coordinate {
            y: c.y.wrapping_sub(1),
            x: c.x,
        };

        self.count_in_sequence(player, coordinate, left_right, right_left) >= 4
            || self.count_in_sequence(
                player,
                coordinate,
                bottom_left_top_right,
                top_right_bottom_left,
            ) >= 4
            || self.count_in_sequence(
                player,
                coordinate,
                top_left_bottom_right,
                bottom_right_top_left,
            ) >= 4
            || self
                .iterate_by_function(top_to_bottom(coordinate), top_to_bottom)
                .take_while(|o| *o == Some(player))
                .count()
                + 1
                >= 4
    }

    /// Takes two stepping functions in opposite directions.
    /// Combines the length of found stones is sequence.
    fn count_in_sequence(
        &self,
        player: Player,
        coordinate: Coordinate,
        f1: impl Fn(Coordinate) -> Coordinate,
        f2: impl Fn(Coordinate) -> Coordinate,
    ) -> usize {
        1 + self
            .iterate_by_function(f1(coordinate), f1)
            .take_while(|o| *o == Some(player))
            .count()
            + self
                .iterate_by_function(f2(coordinate), f2)
                .take_while(|o| *o == Some(player))
                .count()
    }

    /// Generates an iterator of fields from a stepping function.
    fn iterate_by_function<'a>(
        &'a self,
        mut coordinate: Coordinate,
        modification_function: impl Fn(Coordinate) -> Coordinate + 'a,
    ) -> impl Iterator<Item = Option<Player>> + 'a {
        std::iter::from_fn(move || {
            return match self
                .board
                .get(coordinate.y)
                .and_then(|y| y.get(coordinate.x))
            {
                None => None,
                Some(player) => {
                    coordinate = modification_function(coordinate);
                    Some(*player)
                }
            };
        })
    }

    pub fn random_simulation(&self, player: Player) -> f64 {
        let mut current_player = player;
        let mut simulation_board = self.clone();

        while simulation_board.get_game_state() == Ongoing {
            let target_position = simulation_board
                .get_possible_moves()
                .nth(thread_rng().gen_range(0..simulation_board.get_possible_moves().count()))
                .unwrap();
            simulation_board.drop_stone(current_player, target_position);
            match current_player {
                Player::Player1 => current_player = Player2,
                Player::Player2 => current_player = Player1,
            }
        }
        return match simulation_board.get_game_state() {
            GameState::GameResult(Draw) => 0.0,
            GameState::GameResult(GameResult::Win(Player1)) => 1.0,
            GameState::GameResult(GameResult::Win(Player2)) => -1.0,
            Ongoing => {
                unreachable!();
            }
        };
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Player::Player1 => write!(f, "Player 1"),
            Player::Player2 => write!(f, "Player 2"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
    y: usize,
    x: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GameMove {
    x: usize,
}

impl GameMove {
    pub fn new(position: usize) -> GameMove {
        GameMove { x: position }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameState {
    Ongoing,
    GameResult(GameResult),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameResult {
    Draw,
    Win(Player),
}
