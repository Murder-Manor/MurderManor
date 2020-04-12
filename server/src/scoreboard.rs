use std::collections::{
    HashMap,
    HashSet,
};

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
    current_winners: HashSet<Uuid>,
    pub score_board: HashMap<Uuid, u32>,
}

impl ScoreBoard {
    pub fn player_win(&mut self, player_uuid: Uuid) {
        // Don't update the player score board if he already won ;)
        if !self.current_winners.insert(player_uuid) {
            return;
        }
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

    pub fn next_round(&mut self) {
        self.current_state = Rank::First;
        self.current_winners = HashSet::<Uuid>::new();
    }

    pub fn reset(&mut self) {
        self.current_state = Rank::First;
        self.current_winners = HashSet::<Uuid>::new();
        self.score_board = HashMap::<Uuid, u32>::new();
    }
}
