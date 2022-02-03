use crate::connect_four::Player::{Player1, Player2};
use crate::connect_four::{GameBoard, GameMove, GameResult, GameState, Player};
use std::f64::consts::SQRT_2;
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
                "{:?}: {} - {} {}",
                c.game_move,
                c.calculate_uct(self.sample_count as f64),
                c.sample_count,
                c.weight,
            )?;
        }
        writeln!(f, "")
    }
}

impl Node {
    pub fn new(game_board: GameBoard, current_player: Player) -> Node {
        Node {
            game_board,
            current_player,
            game_move: None,
            sample_count: 0,
            children: vec![],
            weight: 0.0,
        }
    }

    pub fn get_board(&self) -> &GameBoard {
        &self.game_board
    }

    pub fn make_move(mut self, game_move: GameMove) -> Option<Node> {
        self.mcts();
        self.mcts();
        self.children
            .into_iter()
            .find(|n| n.game_move == Some(game_move))
    }

    pub fn mcts(&mut self) -> GameResult {
        let result: GameResult;
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
                    result = r;
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
        match result {
            GameResult::Draw => self.weight += 0.0,
            GameResult::Win(p) if p != self.current_player => self.weight += 1.0,
            _ => {}
        }
        result
    }

    pub fn best_move(&mut self) -> GameMove {
        self.mcts();
        self.mcts();
        self.children
            .iter()
            .max_by_key(|c| c.sample_count)
            .unwrap()
            .game_move
            .unwrap()
    }

    fn calculate_uct(&self, parent_sample_count: f64) -> f64 {
        if self.sample_count == 0 {
            return f64::INFINITY;
        }
        self.weight / (self.sample_count as f64)
            + SQRT_2 * ((parent_sample_count.ln() / (self.sample_count as f64)).sqrt())
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
