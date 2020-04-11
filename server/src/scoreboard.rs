use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug)]
pub enum Rank {
    First,
    Second,
    Third,
    Last,
}

impl Default for Rank {
    fn default() -> Self { Rank::First }
}

#[derive(Default, Debug)]
pub struct ScoreBoard {
    current_state: Rank,
    pub score_board: HashMap<Uuid, u32>,
}

impl ScoreBoard {
    pub fn player_win(&mut self, player_uuid: Uuid) {
        let player_score = self.score_board.entry(player_uuid).or_insert(0);

        *player_score += match &self.current_state {
            Rank::First => 5,
            Rank::Second => 3,
            Rank::Third => 2,
            Rank::Last => 0,
        };

        self.current_state = match &self.current_state {
            Rank::First => Rank::Second,
            Rank::Second => Rank::Third,
            Rank::Third => Rank::Last,
            Rank::Last => Rank::Last,
        };

        println!("Player {:} now has {:} points", player_uuid, player_score);
    }

    pub fn reset(&mut self) {
        self.current_state = Rank::First;
        self.score_board = HashMap::<Uuid, u32>::new();
    }
}
