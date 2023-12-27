// use anyhow::{Context, Result};
// use argon2::{password_hash, Argon2, PasswordHash};

// async fn hash_password(password: String) -> Result<String> {
//     Ok(tokio::task::spawn_blocking(move || -> Result<String> {
//         let salt = password_hash::SaltString::generate(rand::thread_rng());
//         Ok(
//             PasswordHash::generate(Argon2::default(), password, salt.as_str())
//                 .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
//                 .to_string(),
//         )
//     })
//     .await
//     .context("panic in generating password hash")??)
// }

// async fn verify_password(password: String, password_hash: String) -> Result<()> {
//     Ok(tokio::task::spawn_blocking(move || -> Result<()> {
//         let hash = PasswordHash::new(&password_hash)
//             .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

//         hash.verify_password(&[&Argon2::default()], password)
//             .map_err(|e| match e {
//                 password_hash::Error::Password => Error::Unauthorized,
//                 _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
//             })
//     })
//     .await
//     .context("panic in verifying password hash")??)
// }
