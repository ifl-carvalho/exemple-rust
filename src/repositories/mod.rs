pub mod user;

use crate::database::postgres;
use crate::repositories::user::{UserRepo, UserRepoImpl};
use axum::Extension;
use std::sync::Arc;

pub type RepoExt = Extension<Arc<RepoImpls>>;

pub async fn create_repositories() -> RepoImpls {
    let db_pool = Arc::new(postgres::db_connect().await);
    RepoImpls::new(UserRepoImpl::new(db_pool.clone()))
}

#[derive(Clone)]
pub struct RepoImpls {
    pub user: UserRepoImpl,
}
impl RepoImpls {
    pub fn new(user_repo_impl: UserRepoImpl) -> Self {
        Self {
            user: user_repo_impl,
        }
    }
}

pub trait Repositories {
    type UserRepoImpl: UserRepo;
    fn user(&self) -> &Self::UserRepoImpl;
}
impl Repositories for RepoImpls {
    type UserRepoImpl = UserRepoImpl;
    fn user(&self) -> &Self::UserRepoImpl {
        &self.user
    }
}
