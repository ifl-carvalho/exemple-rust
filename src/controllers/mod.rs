pub mod middleware;
pub mod recipes;
pub mod user;

pub async fn root() -> &'static str {
    "hello world!"
}
