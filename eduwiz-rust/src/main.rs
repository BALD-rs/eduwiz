use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router, extract::State,
};
use eduwiz_rust::room::Room;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;

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
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/api/create_room", get(create_room))
        .with_state(pool);

    // Run the app on 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
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

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}