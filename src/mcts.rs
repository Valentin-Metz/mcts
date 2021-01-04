use crate::connect_four::Player::{Player1, Player2};
use crate::connect_four::{GameBoard, GameMove, GameResult, GameState, Player};
use std::f64::consts::SQRT_2;
use std::f64::INFINITY;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Node {
    game_board: GameBoard,
    current_player: Player,
    game_move: Option<GameMove>,
    sample_count: u64,
    children: Vec<Node>,
    weight: f64,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in &self.children {
            writeln!(
                f,
                "{:?}: {} - {}",
                c.game_move,
                c.calculate_uct(self.sample_count as f64),
                c.sample_count,
            )?;
        }
        writeln!(f, "")
    }
}

impl Node {
    pub fn new(player: Player) -> Node {
        Node {
            game_board: GameBoard::default(),
            current_player: player,
            game_move: None,
            sample_count: 0,
            children: vec![],
            weight: 0.0,
        }
    }

    pub fn mcts(&mut self) -> f64 {
        let result: f64;
        if self.children.is_empty() {
            match self.game_board.get_game_state() {
                GameState::Ongoing => {
                    if self.sample_count < 1 {
                        // Rollout
                        result = self.game_board.random_simulation(self.current_player);
                    } else {
                        self.children = self.generate_children();
                        result = self.children.first_mut().unwrap().mcts();
                    }
                }
                GameState::GameResult(r) => {
                    result = match r {
                        GameResult::Draw => 0.0,
                        GameResult::Win(Player1) => 1.0,
                        GameResult::Win(Player2) => -1.0,
                    };
                }
            }
        } else {
            let parent_sample_count = self.sample_count as f64;
            let best_child = self
                .children
                .iter_mut()
                .max_by(|c1, c2| {
                    c1.calculate_uct(parent_sample_count)
                        .total_cmp(&c2.calculate_uct(parent_sample_count))
                })
                .unwrap();
            result = best_child.mcts();
        }
        self.sample_count += 1;
        self.weight += result;
        result
    }

    pub fn best_move(&self) -> GameMove {
        self.children
            .iter()
            .max_by(|c1, c2| {
                c1.calculate_uct(self.sample_count as f64)
                    .total_cmp(&c2.calculate_uct(self.sample_count as f64))
            })
            .unwrap()
            .game_move
            .unwrap()
    }

    fn calculate_uct(&self, parent_sample_count: f64) -> f64 {
        if self.sample_count == 0 {
            return INFINITY;
        }
        self.weight / (self.sample_count as f64)
            + 5.0 * SQRT_2 * (parent_sample_count.ln() / (self.sample_count as f64)).sqrt()
    }

    fn generate_children(&self) -> Vec<Node> {
        let mut children = Vec::<Node>::with_capacity(self.game_board.get_dimensions().1);
        children.extend(self.game_board.get_possible_moves().map(move |m| {
            let mut new_board = self.game_board.clone();
            new_board.drop_stone(self.current_player, m);
            Node {
                game_board: new_board,
                current_player: match self.current_player {
                    Player::Player1 => Player2,
                    Player::Player2 => Player1,
                },
                game_move: Some(m),
                sample_count: 0,
                children: Vec::<Node>::with_capacity(self.game_board.get_dimensions().1),
                weight: 0.0,
            }
        }));
        children
    }
}