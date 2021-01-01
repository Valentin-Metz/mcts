use std::fmt;
use std::fmt::Formatter;

use colored::*;

#[derive(Default, Debug)]
pub struct GameBoard {
    board: [[Option<Owner>; 7]; 6],
}

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
                    Some(Owner::Player1) => {
                        write!(f, " {} |", "X".red())?;
                    }
                    Some(Owner::Player2) => {
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

    pub fn get_field(&self, coordinate: Coordinate) -> Option<Owner> {
        self.board[coordinate.y][coordinate.x]
    }

    pub fn place_stone(&mut self, owner: Owner, coordinate: Coordinate) {
        self.board[coordinate.y][coordinate.x] = Some(owner);
    }

    pub fn drop_stone(&mut self, owner: Owner, x_position: usize) -> Option<bool> {
        if x_position >= self.get_dimensions().1 {
            return None;
        }
        for i in 0..self.board.len() {
            if self.board[i][x_position].is_none() {
                self.board[i][x_position] = Some(owner);
                return Some(self.check_win(
                    owner,
                    Coordinate {
                        y: i,
                        x: x_position,
                    },
                ));
            }
        }
        None
    }

    pub fn check_win(&self, owner: Owner, coordinate: Coordinate) -> bool {
        let left_right = |c: Coordinate| Coordinate {
            y: c.y,
            x: c.x.wrapping_add(1),
        };
        let right_left = |c: Coordinate| Coordinate {
            y: c.y,
            x: c.x.wrapping_sub(1),
        };
        let bl_tr = |c: Coordinate| Coordinate {
            y: c.y.wrapping_add(1),
            x: c.x.wrapping_add(1),
        };
        let tr_bl = |c: Coordinate| Coordinate {
            y: c.y.wrapping_sub(1),
            x: c.x.wrapping_sub(1),
        };
        let tl_br = |c: Coordinate| Coordinate {
            y: c.y.wrapping_sub(1),
            x: c.x.wrapping_add(1),
        };
        let br_tl = |c: Coordinate| Coordinate {
            y: c.y.wrapping_add(1),
            x: c.x.wrapping_sub(1),
        };
        let top_bot = |c: Coordinate| Coordinate {
            y: c.y.wrapping_sub(1),
            x: c.x,
        }; // Only needed in one direction
        self.count_in_sequence(owner, coordinate, left_right, right_left) >= 4
            || self.count_in_sequence(owner, coordinate, bl_tr, tr_bl) >= 4
            || self.count_in_sequence(owner, coordinate, tl_br, br_tl) >= 4
            || self
                .iterate_by_function(top_bot(coordinate), top_bot)
                .take_while(|o| *o == Some(owner))
                .count()
                + 1
                >= 4
    }

    fn count_in_sequence(
        &self,
        owner: Owner,
        coordinate: Coordinate,
        f1: impl Fn(Coordinate) -> Coordinate,
        f2: impl Fn(Coordinate) -> Coordinate,
    ) -> usize {
        1 + self
            .iterate_by_function(f1(coordinate), f1)
            .take_while(|o| *o == Some(owner))
            .count()
            + self
                .iterate_by_function(f2(coordinate), f2)
                .take_while(|o| *o == Some(owner))
                .count()
    }

    fn iterate_by_function<'a>(
        &'a self,
        mut coordinate: Coordinate,
        modification_function: impl Fn(Coordinate) -> Coordinate + 'a,
    ) -> impl Iterator<Item = Option<Owner>> + 'a {
        std::iter::from_fn(move || {
            return match self
                .board
                .get(coordinate.y)
                .and_then(|y| y.get(coordinate.x))
            {
                None => None,
                Some(owner) => {
                    coordinate = modification_function(coordinate);
                    Some(*owner)
                }
            };
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Owner {
    Player1,
    Player2,
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Owner::Player1 => write!(f, "Player 1"),
            Owner::Player2 => write!(f, "Player 2"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
    y: usize,
    x: usize,
}
