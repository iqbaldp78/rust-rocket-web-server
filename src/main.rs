#![feature(proc_macro_hygiene, decl_macro)]

// use std::io;

// use rocket::tokio::task::spawn_blocking;
// use rocket::tokio::time::{sleep, Duration};
use serde::{Serialize, Deserialize};
use rust_rocket_web_server::rocket_builder;
mod routes;

// #[get("/delay/<seconds>")]
// async fn delay(seconds: u64) -> String {
//     sleep(Duration::from_secs(seconds)).await;
//     format!("Waited for {} seconds", seconds)
// }

// #[get("/blocking_task")]
// async fn blocking_task() -> io::Result<Vec<u8>> {
//     // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
//     let vec = spawn_blocking(|| std::fs::read("data.txt")).await
//         .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

//     Ok(vec)
// }

fn rocket() -> rocket::Rocket {
    rocket_builder()
}

fn main() {
    rocket().launch();
}

#[derive(Serialize, Deserialize)]
struct BlogPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}