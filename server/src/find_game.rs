use std::{
    time,
    error,
    fmt
};
use std::sync::Arc;
use std::time::SystemTime;

use tokio::time::delay_for;
use tokio::sync::Mutex;

use crate::proto::{
    Player,
    Vector3,
};

use crate::proto::game_progress::Status as ProtoGameStatus;

use crate::players::Players;
use crate::objects::Objects;
use crate::scoreboard::ScoreBoard;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GenericError;

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Internal error")
    }
}

impl error::Error for GenericError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameStatus {
    WaitingForPlayers,
    StartCountdown(SystemTime),
    InGame(u8),
    ScoreBoard,
}

impl Default for GameStatus {
    fn default() -> Self { GameStatus::WaitingForPlayers }
}

impl GameStatus {
    pub fn to_proto(&self) -> i32 {
        match self {
            GameStatus::WaitingForPlayers => ProtoGameStatus::WaitingForPlayers as i32,
            GameStatus::StartCountdown(_) => ProtoGameStatus::StartCountdown as i32,
            GameStatus::InGame(_) => ProtoGameStatus::InGame as i32,
            GameStatus::ScoreBoard => ProtoGameStatus::ScoreBoard as i32,
        }
    }
}

#[derive(Default)]
pub struct GameStateMachine {
    pub game_state: GameStatus,
    pub object_to_take: Option<Uuid>,
}

#[derive(Default)]
pub struct GameCore {
    pub game_state_machine: Arc<Mutex<GameStateMachine>>,
    pub max_players: i8,
    pub players: Arc<Mutex<Players>>,
    pub objects: Arc<Mutex<Objects>>,
    pub score_board: Arc<Mutex<ScoreBoard>>,
}

impl GameCore {
    pub fn start(&mut self) {
        let players = self.players.clone();
        tokio::spawn(async move {
            loop {
                players.lock().await.remove_dead_players();
                delay_for(time::Duration::from_millis(100)).await;
            }
        });

        let state_machine = self.game_state_machine.clone();
        let objects = self.objects.clone();
        let max_players = self.max_players.clone();
        tokio::spawn(async move {
            loop {
                delay_for(time::Duration::from_millis(100)).await;
                let game_state = state_machine.lock().await.game_state;
                match game_state {
                    GameStatus::WaitingForPlayers => continue,
                    GameStatus::StartCountdown(start_time) => {
                        delay_for(
                            start_time.duration_since(SystemTime::now()).unwrap())
                            .await;
                        println!("Starting game now!");

                        state_machine.lock().await.object_to_take =
                            Some(objects.lock().await.take_random_takable_object());

                        state_machine.lock().await.game_state = GameStatus::InGame(0);
                    },
                    GameStatus::InGame(_round) => {
                        let takers = objects.lock().await
                            .takers_for(state_machine.lock().await.object_to_take.unwrap());
                        if takers.len() >= max_players as usize {
                            state_machine.lock().await.game_state = GameStatus::ScoreBoard;
                        }
                    },
                    GameStatus::ScoreBoard => {
                        println!("Game finished");
                    },
                }
            }
        });

        // Reset game state if all the players left the game
        let players = self.players.clone();
        let state_machine = self.game_state_machine.clone();
        let score_board = self.score_board.clone();
        tokio::spawn(async move {
            loop {
                if players.lock().await.internal_players.keys().len() == 0 {
                    if state_machine.lock().await.game_state != GameStatus::WaitingForPlayers {
                        println!("No more player, resetting game state");
                        state_machine.lock().await.game_state = GameStatus::WaitingForPlayers;
                        score_board.lock().await.reset();
                    }
                }
            }
        });
    }

    pub async fn new_player(&mut self, player_uuid: Uuid, name: String) -> Result<Player, GenericError> {
        if self.game_state_machine.lock().await.game_state != GameStatus::WaitingForPlayers {
            return Err(GenericError)
        }

        let update_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let player = Player {
            id: player_uuid.to_hyphenated().to_string(),
            name: name,
            role: crate::proto::player::Role::Wolf as i32,
            position: Some(Vector3::default()),
            direction: Some(Vector3::default()),
            last_updateds: update_time,
        };

        self.players.lock().await.internal_players.insert(player_uuid, player.clone());

        // As soon as we reached our maximum number of players, start the countdown!
        if self.players.lock().await
            .internal_players.keys().len() >= self.max_players as usize {
                let st = SystemTime::now()
                    .checked_add(time::Duration::from_secs(5))
                    .unwrap();
                self.game_state_machine.lock().await.game_state = GameStatus::StartCountdown(st);
            }

        Ok(player)
    }

    pub async fn take_object(&mut self, object_uuid: Uuid, player_uuid: Uuid) -> Result<(), GenericError> {
        println!("{:} took {:}", player_uuid, object_uuid);
        // Take the object physically
        self.objects.lock().await
            .take_object(object_uuid, player_uuid)
            .unwrap();

        // In case it was the object to find, update the scoreboard
        if object_uuid == self.game_state_machine.lock().await
            .object_to_take.unwrap_or_default() {
                self.score_board.lock().await.player_win(player_uuid);
        }
        Ok(())
    }
}

