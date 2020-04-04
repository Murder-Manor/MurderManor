use std::collections::HashMap;
use std::time::{
    SystemTime,
    Duration
};

use uuid::Uuid;

use crate::proto::Player;

#[derive(Default)]
pub struct Players {
    pub internal_players: HashMap<Uuid, Player>,
}

impl Players {
    pub fn remove_dead_players(&mut self) {
        let mut to_delete = Vec::new();
        for (k, player) in self.internal_players.iter() {
            let last_update = SystemTime::UNIX_EPOCH
                .checked_add(Duration::from_secs(player.last_updateds))
                .unwrap()
                .elapsed()
                .unwrap();
            if last_update > Duration::from_secs(2) {
                println!("Will delete {:}, last updated {:?} ago", k, last_update);
                to_delete.push(k.clone());
            }
        }

        // Cleanup to_delete resources
        for del in to_delete {
            self.internal_players.remove(&del);
        }
    }
}
