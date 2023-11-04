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

use eduwiz_rust::room::{Room, Question};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{net::SocketAddr, time::Duration, collections::HashSet, sync::Arc};

use dotenvy::dotenv;

use r2d2_redis::{r2d2::{self, Pool}, RedisConnectionManager};
use r2d2_redis::redis::{Commands, RedisResult};
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
        .allow_origin(Any);


    // Application built
    let app = Router::new()
        .route("/api/create_room", get(create_room))
        .route("/api/start_room/:room", get(start_room))
        .route("/api/join_room/:room", get(join_room))
        .route("/api/submit_answer", post(submit_answer))
        .with_state(pool)
        .layer(cors);

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
    loop {
        interval.tick().await;
    }
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
    let mut interval = tokio::time::interval(Duration::from_secs(2));
    let mut conn = pool.get().unwrap();
    loop {
        // Executes every 2 seconds
        interval.tick().await;
        // Polls for latest room
        let room: Room = match conn.get(&room) {
            Ok(room) => room,
            Err(_) => {
                println!("Failed to get room");
                continue;
            }
        };

        if room.get_finished() {
            socket.send(Message::Text(String::from("END"))).await;
        }

        if room.get_started() {
            socket.send(Message::Text(String::from("START"))).await;
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

    let room: Room = match conn.get(&payload.room) {
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
            let user_score = user_score.unwrap();
            let _: () = conn.set(&payload.user.to_string(), user_score).unwrap();
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
