use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use dotenvy::dotenv;

use redis::{Commands, RedisResult};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    
    let redis_username = std::env::var("USERNAME").expect("USERNAME must be set.");
    let redis_password = std::env::var("PASSWORD").expect("PASSWORD must be set.");
    let redis_host = std::env::var("HOST").expect("HOST must be set.");

    let url = format!("redis://{redis_username}:{redis_password}@{redis_host}");
    let client = redis::Client::open(url).unwrap();
    let mut con = client.get_connection().unwrap();
    
    let _ : () = con.set("my_key", 42).unwrap();
    let keyval: RedisResult<isize> = con.get("my_key");
    println!("{:?}", keyval);
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
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