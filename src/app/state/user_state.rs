use std::sync::Arc;

use crate::{app::usecase::user_usecase::UserUsecase, infrastructure::db::postgres::Database};


#[derive(Clone)]
pub struct UserState {
    pub user_usecase: UserUsecase
}

impl UserState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_usecase: UserUsecase::new(db_conn),
        }
    }
}