/// GameBoard - implemented as a 2D struct array.
/// A field can be empty (None) - or occupied by one of tho two players (Some(Player)).
/// Default size is 3 long (indexed by x); 3 high (indexed by y).
#[derive(Debug, Copy, Clone)]
pub struct GameBoard {
    board: [[Option<Player>; 3]; 3],
}

/// Pretty prints the GameBoard. Uses ANSI-Escape-Codes for color and in-place prints.
impl std::fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "| 0 | 1 | 2 |\n")?;
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
