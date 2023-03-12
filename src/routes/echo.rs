use rocket::*;

#[get("/echo/<echo>")]
pub fn echo_fn(echo: String) -> String {
    echo
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, worlds!"
}

#[get("/ping")]
pub fn ping_fn() -> String {
    "PONG!".to_string()
}