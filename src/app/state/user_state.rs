use std::sync::Arc;

use crate::infrastructure::{db::postgres::Database, repositories::user_repositories::{UserRepository, UserRepositoryTrait}};


#[derive(Clone)]
pub struct UserState {
    pub user_repository: UserRepository,
}

impl UserState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_repository: UserRepository::new(db_conn),
        }
    }
}