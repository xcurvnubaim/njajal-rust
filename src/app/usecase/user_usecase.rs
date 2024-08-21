extern crate bcrypt;
use crate::{
    app::response::error::ErrorResponse,
    domain::entities::user_entities::GetUser,
    infrastructure::{
        db::postgres::Database,
        repositories::user_repositories::{UserRepository, UserRepositoryTrait},
    }, presentation::dto::token::Claims,
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use core::time;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserUsecase {
    pub user_repository: UserRepository,
}

impl UserUsecase {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_repository: UserRepository::new(db_conn),
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<GetUser>, ErrorResponse> {
        match self.user_repository.get_all_users().await {
            Ok(users) => Ok(users),
            Err(err) => Err(ErrorResponse::new(
                "Failed to get all users".to_string(),
                err.to_string(),
            )),
        }
    }

    pub async fn create_user(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<GetUser, ErrorResponse> {
        // Hash the password
        let hashed_password = match hash(password, DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(_) => {
                return Err(ErrorResponse::new(
                    "Failed to hash password".to_string(),
                    "HashError".to_string(),
                ))
            }
        };

        // Attempt to create the user
        match self
            .user_repository
            .create_user(name, email, hashed_password)
            .await
        {
            Ok(user) => Ok(user), // Return the user if successful
            Err(err) => Err(ErrorResponse::new(
                "Failed to create user".to_string(),
                err.to_string(),
            )), // Return error if there's an issue
        }
    }

    pub async fn login_user(&self, email: String, password: String) -> Result<(GetUser, String), ErrorResponse> {
        // Find the user by email
        let user = match self.user_repository.find_by_email(email).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err(ErrorResponse::new(
                    "User not found".to_string(),
                    "UserNotFound".to_string(),
                ))
            }
            Err(_) => {
                return Err(ErrorResponse::new(
                    "Failed to find user".to_string(),
                    "FindUserError".to_string(),
                ))
            }
        };
    
        // Check if the password matches
        if let Err(_) | Ok(false) = bcrypt::verify(password, &user.password) {
            return Err(ErrorResponse::new(
                "Invalid password".to_string(),
                "InvalidPassword".to_string(),
            ));
        }
    
        // Generate Token
        let my_claims = Claims {
            id: user.id,
            role: "user".to_string(),
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
        };
        
        let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())) {
            Ok(token) => token,
            Err(_) => {
                return Err(ErrorResponse::new(
                    "Failed to generate token".to_string(),
                    "TokenError".to_string(),
                ))
            }
        };

        // Return the user with the claims or token (if you have implemented that)
        Ok((user, token))
    }
    
    pub async fn get_me(&self, id: i32) -> Result<GetUser, ErrorResponse> {
        match self.user_repository.find_by_id(id).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(ErrorResponse::new(
                "User not found".to_string(),
                "UserNotFound".to_string(),
            )),
            Err(err) => Err(ErrorResponse::new(
                "Failed to find user".to_string(),
                err.to_string(),
            )),
        }
    }
}
