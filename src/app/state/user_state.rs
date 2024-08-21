use std::sync::Arc;

use crate::{app::usecase::user_usecase::UserUsecase, infrastructure::{db::postgres::Database, repositories::user_repositories::{UserRepository, UserRepositoryTrait}}};

use super::token_state::TokenState;


#[derive(Clone)]
pub struct UserState {
    pub user_usecase: UserUsecase,
    pub token_state: TokenState,
}

impl UserState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_usecase: UserUsecase::new(db_conn),
            token_state: TokenState::new(),
        }
    }
}