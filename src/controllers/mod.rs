pub mod health_check;
pub mod user;

pub async fn root() -> &'static str {
    "hello world!"
}
