use entity::entities::users;
use sea_orm::DatabaseConnection;
use service::users::{mutation, query};

use super::{auth_server::Auth, LoginRequest, LoginResponse, RegisterRequest};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct AuthService {
    pub db_connection: DatabaseConnection,
}

impl RegisterRequest {
    pub fn into(self) -> users::Model {
        users::Model {
            id: 0,
            username: self.username,
            password: self.password,
            created_at: Default::default(),
            updated_at: None,
        }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!("Got a request: {:?}", request);
        let r = request.into_inner();
        let db_result = query::Query::find_user_by_username_password(
            &self.db_connection,
            &r.username,
            &r.password,
        )
        .await;
        if db_result.is_err() {
            return Err(Status::invalid_argument("Invalid username or password"));
        }
        let db_result = db_result.unwrap();
        if db_result.is_none() {
            return Err(Status::invalid_argument("Invalid username or password"));
        }
        let db_result = db_result.unwrap();
        let response = super::LoginResponse {
            token: db_result.id.to_string(),
        };
        Ok(Response::new(response))
    }

    async fn register(
        &self,
        request: Request<super::RegisterRequest>,
    ) -> Result<Response<super::RegisterResponse>, Status> {
        println!("Got a request: {:?}", request);
        let r = request.into_inner();
        let db_result = mutation::Mutation::create_user(&self.db_connection, r.into()).await;
        if db_result.is_err() {
            return Err(Status::invalid_argument("Invalid username or password"));
        }
        let db_result = db_result.unwrap();
        let response = super::RegisterResponse {
            token: db_result.id.to_string(),
        };
        Ok(tonic::Response::new(response))
    }
}
