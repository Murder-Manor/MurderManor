use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::{
    mpsc,
    Mutex,
};
use tonic::{
    Request,
    Response,
    Status,
    Code,
};

use crate::proto::extra_server::Extra;
use crate::proto::{
    ServiceInfoRequest,
    ServiceInfoReply,
};

use crate::proto::game_server::Game;
use crate::proto::{
    GetGameProgressRequest,
    ResetRequest,
    NewPlayerRequest,
    GetPlayerRequest,
    ListPlayersRequest,
    MovePlayerRequest,
    TakeObjectRequest,
    GetScoreBoardRequest,
    GameProgress,
    ResetResponse,
    Player,
    ObjectStatus,
    PlayerScore,
    ScoreBoard,
};

use crate::find_game::{
    GameCore,
    GameStatus
};

use uuid::Uuid;

macro_rules! parse_uuid_or_fail {
    ($s:expr) => {
        Uuid::parse_str(&String::from(&$s))
            .map_err(|_| Status::new(Code::FailedPrecondition, "Wrong UUID format"))?
    };
}

fn uuid_to_string(id: Uuid) -> String {
    id.to_hyphenated().to_string()
}

pub struct ExtraAPI{
}

#[tonic::async_trait]
impl Extra for ExtraAPI {
    async fn service_info(&self,
                          request: Request<ServiceInfoRequest>
                         ) ->
        Result<Response<ServiceInfoReply>, Status> {
            println!("Got a server status request: {:?}", request);

            let reply = ServiceInfoReply {
                ready: true,
            };

            Ok(Response::new(reply))
        }
}

pub struct GameAPI{
    pub core: Arc<Mutex<GameCore>>,
}

#[tonic::async_trait]
impl Game for GameAPI {
    async fn get_game_progress(&self,
                               _request: Request<GetGameProgressRequest>
                               ) ->
        Result<Response<GameProgress>, Status> {
            let core = self.core.lock().await;
            let sm = core.game_state_machine.lock().await;
            let progress = match sm.game_state {
                GameStatus::WaitingForPlayers => GameProgress {
                    status: sm.game_state.to_proto(),
                    ..Default::default()
                },
                GameStatus::StartCountdown(st) => GameProgress {
                    status: sm.game_state.to_proto(),
                    milliseconds_before_start: if SystemTime::now() < st {
                        st.duration_since(SystemTime::now()).unwrap().as_millis() as u32
                    } else { 0 },
                    ..Default::default()
                },
                GameStatus::InGame(round, object_to_take) => GameProgress {
                        status: sm.game_state.to_proto(),
                        object_to_take: uuid_to_string(object_to_take),
                        current_round: round as u32,
                        ..Default::default()
                },
                GameStatus::CountDownTilNextRound(st, round) => GameProgress {
                        status: sm.game_state.to_proto(),
                        milliseconds_before_start: if SystemTime::now() < st {
                            st.duration_since(SystemTime::now()).unwrap().as_millis() as u32
                        } else { 0 },
                        current_round: round as u32,
                        ..Default::default()
                },
                GameStatus::ScoreBoard => GameProgress {
                    status: sm.game_state.to_proto(),
                    ..Default::default()
                },
            };
            Ok(Response::new(progress))
        }

    async fn reset(&self,
                   _: Request<ResetRequest>
                       ) ->
        Result<Response<ResetResponse>, Status> {
            self.core.lock().await.reset().await;
            Ok(Response::new(ResetResponse::default()))
        }

    async fn new_player(&self,
                        request: Request<NewPlayerRequest>
                       ) ->
        Result<Response<Player>, Status> {
            let player_uuid = Uuid::new_v4();

            let player = self.core
                .lock().await
                .new_player(player_uuid, request.into_inner().name).await
                .map_err(|_| Status::new(Code::Internal, "internal error"))?;

            println!("New player: {:?}", player);

            Ok(Response::new(player))
        }

    async fn get_player(&self,
                        request: Request<GetPlayerRequest>
                       ) ->
        Result<Response<Player>, Status> {
            let player_uuid = parse_uuid_or_fail!(request.into_inner().id);

            let player = self.core
                .lock().await
                .players.lock().await
                .internal_players.get(&player_uuid)
                .map_or(Player::default(), |player| player.clone());

            Ok(Response::new(player))
        }

    type ListPlayersStream = mpsc::Receiver<Result<Player, Status>>;

    async fn list_players(&self,
                          _request: Request<ListPlayersRequest>
                         ) ->
        Result<Response<Self::ListPlayersStream>, Status> {
            let orig_players = self.core.lock().await.players.clone();
            let (mut tx, rx) = mpsc::channel(4);

            tokio::spawn(async move {
                let players = &orig_players.lock().await.internal_players;
                for (_, player) in players.iter() {
                    tx.send(Ok(player.clone())).await.unwrap();
                }
            });

            Ok(Response::new(rx))
        }

    async fn move_player(&self,
                         request: Request<MovePlayerRequest>
                        ) ->
        Result<Response<Player>, Status> {

            let request = request.into_inner();
            let player_uuid = parse_uuid_or_fail!(request.id);
            let update_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|t| t.as_secs())
                .map_err(|e| Status::new(Code::Internal, format!("internal error: {}", e)))?;

            self.core.lock().await
                .players.lock().await
                .internal_players.get_mut(&player_uuid)
                .ok_or(Status::new(Code::Internal, "Cannot fetch player"))
                .map(|player| {
                    player.position = request.position;
                    player.direction = request.direction;
                    player.last_updateds = update_time;
                    Response::new(player.clone())
                })
        }

    async fn take_object(&self,
                         request: Request<TakeObjectRequest>
                         ) ->
        Result<Response<ObjectStatus>, Status> {
            let request = request.into_inner();
            let player_uuid = parse_uuid_or_fail!(request.player_id);
            let object_uuid = parse_uuid_or_fail!(request.object_id);
            println!("{:} took {:}", player_uuid, object_uuid);
            self.core.lock().await
                .take_object(object_uuid, player_uuid).await
                .unwrap_or_else(|e| println!(
                        "User {:} cannot take {:}: {:}", player_uuid, object_uuid, e));
            Ok(Response::new(ObjectStatus::default()))
        }

    async fn get_score_board(&self,
                             _: Request<GetScoreBoardRequest>
                            ) ->
        Result<Response<ScoreBoard>, Status> {
            let score = self.core.lock().await.score_board.lock().await.score_board.clone();
            let resp = ScoreBoard {
                players: score.iter().map(|(&id, &sc)| PlayerScore{
                    player_id: uuid_to_string(id),
                    score: sc,
                }).collect(),
            };
            Ok(Response::new(resp))
        }
}
