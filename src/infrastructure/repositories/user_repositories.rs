use std::sync::Arc;

use axum::async_trait;
use sqlx::query_as;
use sqlx::Error;

use crate::domain::entities::user_entities::GetUser;
use crate::infrastructure::db::postgres::{Database, DatabaseTrait};


// use crate::domain::entities::user_entities::User;


#[derive(Clone)]
pub struct UserRepository {
    pub(crate) db_conn: Arc<Database>,
}

#[async_trait]
pub trait UserRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;
    // async fn find_by_email(&self, email: String) -> Option<User>;
    // async fn find(&self, id: u64) -> Result<User, Error>;
    async fn get_all_users(&self) -> Result<Vec<GetUser>, Error>;
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }
    
    async fn get_all_users(&self) -> Result<Vec<GetUser>, sqlx::Error> {
        // Define the query
        let query = "SELECT ida, name, email, password, created_at, updated_at FROM users";
        
        // Execute the query and fetch all rows
        let users = query_as::<_, GetUser>(query)
            .fetch_all(self.db_conn.get_pool())
            .await;
        
        return users;
    }
}
