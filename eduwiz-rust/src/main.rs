use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router, extract::{State, Path, ConnectInfo},
};
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};

use eduwiz_rust::room::{Room, Question};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{net::SocketAddr, time::Duration, collections::HashSet};

use dotenvy::dotenv;

use r2d2_redis::{r2d2::{self, Pool}, RedisConnectionManager};
use r2d2_redis::redis::{Commands, RedisResult};

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

    // Application built
    let app = Router::new()
        .route("/api/create_room", get(create_room))
        .route("/api/start_room/:room", get(start_room))
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
struct CreateRoomResponse {
    room_code: String,
}

// Creates a new room and returns the room code
async fn create_room(
    State(pool): State<Pool<RedisConnectionManager>>,
) -> Result<Json<CreateRoomResponse>, StatusCode> {
    let new_room = Room::new();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => todo!()//return Err(StatusCode::INTERNAL_SERVER_ERROR) 
    };
    let room_code = new_room.get_code();
    let room_string = json!(new_room).to_string();
    let _: () = conn.set(room_code, room_string).unwrap();
    return  Ok( Json(CreateRoomResponse { room_code: new_room.get_code() }) );
}

// Starts room given and upgrades to a websocket
async fn start_room(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(room): Path<String>,
    State(pool): State<Pool<RedisConnectionManager>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_host_socket(socket, addr, axum::extract::State(pool)))
}

async fn handle_host_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    State(pool): State<Pool<RedisConnectionManager>>,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(5));
}

// Starts room given and upgrades to a websocket
async fn join_room(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(room): Path<String>,
    State(pool): State<Pool<RedisConnectionManager>>,
) -> impl IntoResponse {
    
    ws.on_upgrade(move |socket| handle_client_socket(room, socket, addr, axum::extract::State(pool)))
}

async fn handle_client_socket(
    room: String,
    mut socket: WebSocket,
    who: SocketAddr,
    State(pool): State<Pool<RedisConnectionManager>>,
) {
    let mut conn = pool.get().unwrap();
    loop {
        // Polls for latest room
        let room: Room = match conn.get(&room) {
            Ok(room) => room,
            Err(_) => {
                println!("Failed to get room");
                continue;
            }
        };
        // Gets new question to submit
        let new_question = room.new_question();
        let client_question = json!(ClientQuestion {
            prompt: new_question.prompt,
            answers: new_question.answers,
        });
        // Sends new question to client
        if let Err(e) = socket.send(Message::Text(client_question.to_string())).await {
            println!("Failed to send message: {}", e);
        }
        let message = socket.recv().await;
        
    }
}

#[derive(Serialize)]
pub struct ClientQuestion {
    prompt: String,
    answers: HashSet<String>
}

#[derive(Serialize)]
enum ToClientMessage {
    NewQuestion(ClientQuestion),
    AnsweredCorrect(bool),
    EndGame
}

// array o open connections