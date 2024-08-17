use sqlx::query_as;
use sqlx::PgPool;
use crate::domain::entities::user_entities::User;


// #[derive(Clone)]
// pub struct UserRepository {
//     pub(crate) db_conn: Arc<Database>,
// }

// #[async_trait]
// pub trait UserRepositoryTrait {
//     fn new(db_conn: &Arc<Database>) -> Self;
//     async fn find_by_email(&self, email: String) -> Option<User>;
//     async fn find(&self, id: u64) -> Result<User, Error>;
// }

pub async fn get_all_users(db: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    // Fetch all users from the database
    let users = query_as!(
        User,
        "SELECT id, name, email FROM users"
    )
    .fetch_all(db)
    .await?;
    
    Ok(users)
}
