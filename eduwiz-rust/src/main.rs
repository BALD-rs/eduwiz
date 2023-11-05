use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
        Json as Json2
    },
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router, extract::{State, Path, ConnectInfo, Extension, self},
};
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};

use eduwiz_rust::room::{Room, Question, User};
use serde::{Deserialize, Serialize};
use serde_json::json;
use core::time;
use std::{net::SocketAddr, time::Duration, collections::HashSet, sync::Arc};

use dotenvy::dotenv;

use r2d2_redis::{r2d2::{self, Pool}, RedisConnectionManager, redis::{ConnectionLike, Value, FromRedisValue}};
use r2d2_redis::redis::{Commands, RedisResult};
use r2d2_redis::redis::{ Cmd};

use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    
    let redis_username = std::env::var("USERNAME").expect("USERNAME must be set.");
    let redis_password = std::env::var("PASSWORD").expect("PASSWORD must be set.");
    let redis_host = std::env::var("HOST").expect("HOST must be set.");

    let url = format!("redis://{redis_username}:{redis_password}@{redis_host}");
    let manager = RedisConnectionManager::new(url).unwrap();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap();
    
    let mut con = pool.get().unwrap();
    
    let _ : () = con.set("my_key", 42).unwrap();
    
    let keyval: RedisResult<isize> = con.get("my_key");
    println!("{:?}", keyval);
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_private_network(true);

    // Application built
    let app = Router::new()
        .route("/api/create_room", post(create_room))
        .route("/api/start_room/:room", get(start_room))
        .route("/api/join_room/:room/:user", get(join_room))
        .route("/api/get_users/:room", get(get_users))
        .route("/api/submit_answer", post(submit_answer))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // Run the app on 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct RoomUsers {
    users: Vec<String>,
}

pub async fn get_users (
    Path(room): Path<String>,
    State(pool): State<Pool<RedisConnectionManager>>,
) -> Result<Json<RoomUsers>, StatusCode>{
    let mut conn = pool.get().unwrap();
    let r = get_room(&room, pool).await.unwrap();
    let users = r.get_usernames();
    return Ok(Json(RoomUsers { users: users }));
}


#[derive(Serialize, Deserialize)]
struct CreateRoomResponse {
    room_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGameRequest {
    questions: Vec<Question>,
    time_limit: i32,
}

async fn create_room(
    State(pool): State<Pool<RedisConnectionManager>>,
    extract::Json(payload): extract::Json<CreateGameRequest>,
) -> Result<Json<CreateRoomResponse>, StatusCode> {
    let mut new_room = Room::new();

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => todo!()//return Err(StatusCode::INTERNAL_SERVER_ERROR) 
    };

    for question in payload.questions {
        new_room.add_question(question);
    }

    new_room.set_time(payload.time_limit);

    let room_code = new_room.get_code();
    let room_string = json!(new_room).to_string();
    // Sets Redis database to current room json
    let cmd = redis::cmd("JSON.SET").arg(&[room_string.clone(), json!(new_room).to_string()]);
    let mut cmd = Cmd::new();
    cmd.arg("JSON.SET").arg(room_code).arg("$").arg(json!(new_room).to_string());

    let apple = conn.req_command(&cmd).unwrap();
    println!("{:?}", apple);
    return  Ok( Json(CreateRoomResponse { room_code: new_room.get_code() }) );
}

// Starts room given and upgrades to a websocket
async fn start_room(
    ws: WebSocketUpgrade,
    Path(room): Path<String>,
    State(pool): State<Pool<RedisConnectionManager>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_host_socket(room, socket, pool))
}

async fn handle_host_socket(
    room: String,
    mut socket: WebSocket,
    pool: Pool<RedisConnectionManager>,
) {
    let mut conn = pool.get().unwrap();
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut time = 0;
    // Starts and gets room from redis database
    let mut r = get_room(&room, pool.clone()).await.unwrap();
    r.start_room();
    // Updates room on redis end
    let mut cmd = Cmd::new();
    cmd.arg("JSON.SET").arg(&room).arg("$").arg(json!(r).to_string());
    let apple = conn.req_command(&cmd).unwrap();
    loop {
        interval.tick().await;
        time += 1;
        let mut r = get_room(&room, pool.clone()).await.unwrap();

        let mut user_and_score = Vec::new();
        for user in r.get_usernames() {
            let score: i32 = conn.get(&user).unwrap_or(0);
            user_and_score.push((user.clone(), score));
        }

        let _: () = socket.send(Message::Text(json!(user_and_score).to_string())).await.unwrap();

        if time > r.get_time_limit() {
            // Updates room from Redis database
            let mut r = get_room(&room, pool.clone()).await.unwrap();
            // Ends room
            r.end_room();

            let mut cmd = Cmd::new();
            cmd.arg("JSON.SET").arg(&room).arg("$").arg(json!(r).to_string());
            let apple = conn.req_command(&cmd).unwrap();
            break;
        }
    }
}

async fn get_room(room: &String, pool: Pool<RedisConnectionManager>) -> Result<Room,()> {
    let mut conn = pool.get().unwrap();
    let mut cmd = Cmd::new();
        cmd.arg("JSON.GET").arg(room.clone()).arg("$");
        let json_req = conn.req_command(&cmd);

        let mut room: String = match json_req {
            Ok(value) => {
                match String::from_redis_value(&value) {
                    Ok(room_json) => room_json,
                    Err(e) => return Err(())
                }
            }
            Err(e) => {
                println!("{}", e);
                println!("Failed to get room");
                return Err(());
            }
        };

        room.pop();
        room.remove(0);

        let r: Room = serde_json::from_str(&room).unwrap();
        return Ok(r);
}

// Starts room given and upgrades to a websocket
async fn join_room(
    ws: WebSocketUpgrade,
    //ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path((room, user)): Path<(String, String)>,
    State(pool): State<Pool<RedisConnectionManager>>,
) -> impl IntoResponse {
    let mut conn = pool.get().unwrap();
    let mut r = get_room(&room, pool.clone()).await.unwrap();
    let new_user = User {
        id: 7,
        name: user.clone(),
    };
    
    r.add_user(new_user);
    let mut cmd = Cmd::new();
    cmd.arg("JSON.SET").arg(&room).arg("$").arg(json!(r).to_string());
    let apple = conn.req_command(&cmd).unwrap();
    println!("joined");
    ws.on_upgrade(move |socket| handle_client_socket(room, socket, pool))
}

async fn handle_client_socket(
    room: String,
    mut socket: WebSocket,
    //who: SocketAddr,
    pool: Pool<RedisConnectionManager>,
) {
    println!("one client joined");
    let mut interval = tokio::time::interval(Duration::from_secs(2));
    let mut conn = pool.get().unwrap();
    let mut finished_sent = false;
    let mut started_sent = false;
    loop {
        // Executes every 2 seconds
        interval.tick().await;
        // Grabs latest room data
        let r = get_room(&room, pool.clone()).await.unwrap();

        if r.get_finished() && !finished_sent{
            let _: () = socket.send(Message::Text(String::from("END"))).await.unwrap();
            finished_sent = true;
        }

        if r.get_started() && !started_sent{
            let _: () = socket.send(Message::Text(String::from("START"))).await.unwrap();
            started_sent = true;
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct QuestionInfo {
    pub user: String,
    pub room: String,
    pub question: String,
    pub answer: String,
}

async fn submit_answer(
    State(pool): State<Pool<RedisConnectionManager>>,
    extract::Json(payload): extract::Json<QuestionInfo>
) -> Result<Json<ClientQuestion>, StatusCode>{
    
    let mut conn = pool.get().unwrap();

    let room: Room = match get_room(&payload.room, pool).await {
        Ok(room) => room,
        Err(_) => {
            println!("Failed to get room");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let backend_questions = room.get_questions();
    let mut last_correct = false;
    if let Some(b_q) = backend_questions.get(&payload.question) {
        if b_q.check_correct(payload.answer) {
            let user_score: RedisResult<isize> = conn.get(payload.user.clone());
            match user_score {
                Ok(score) => {
                    let _: () = conn.set(&payload.user.to_string(), score + 1).unwrap();
                }
                Err(_) => {
                    let _: () = conn.set(&payload.user.to_string(), 1).unwrap();
                }
            }
            last_correct = true;
        }
    }

    let new_question = room.new_question();
    let client_question = ClientQuestion {
        prompt: new_question.prompt,
        answers: new_question.answers,
        last_correct,
    };
    //return 2;
    return Ok(Json(client_question));
}

#[derive(Serialize, Deserialize)]
pub struct ClientQuestion {
    prompt: String,
    answers: HashSet<String>,
    last_correct: bool,
}
