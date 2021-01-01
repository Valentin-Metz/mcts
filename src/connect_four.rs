use std::fmt;
use std::fmt::Formatter;

use colored::*;

/// GameBoard - implemented as a 2D struct array.
/// A field can be empty (None) - or occupied by one of tho two players (Some(Player)).
/// Default size is 7 long (indexed by x); 6 high (indexed by y).
#[derive(Default, Debug)]
pub struct GameBoard {
    board: [[Option<Player>; 7]; 6],
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
        }
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

    /// Places a stone from player p at a specific coordinate.
    /// Coordinate must be valid.
    pub fn place_stone(&mut self, player: Player, coordinate: Coordinate) {
        self.board[coordinate.y][coordinate.x] = Some(player);
    }

    /// Drops a stone into a specified column (from the top).
    /// If the coordinate is invalid, None will be returned.
    /// If the coordinate is valid, the returned value indicates:
    /// A: The current player has won after this move (Some(true)).
    /// B: The game goes on (Some(false)) - (this includes a potential draw).
    pub fn drop_stone(&mut self, player: Player, x_position: usize) -> Option<bool> {
        if x_position >= self.get_dimensions().1 {
            return None;
        }
        for i in 0..self.board.len() {
            if self.board[i][x_position].is_none() {
                self.board[i][x_position] = Some(player);
                return Some(self.check_win(
                    player,
                    Coordinate {
                        y: i,
                        x: x_position,
                    },
                ));
            }
        }
        None
    }

    /// Checks if no further stones can be placed on the board.
    pub fn check_draw(&self) -> bool {
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
