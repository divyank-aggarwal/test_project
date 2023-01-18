use crate::{
    errors::ServiceError,
    models::{User, UserCounter},
    Users,
};
use actix_web::{
    delete, get, post,
    web::{self, Json},
    Responder,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Debug, Serialize, Deserialize)]
struct AddUserRequest {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AddUserResponse {
    pub result: String,
    pub user_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeleteUserResponse {
    pub result: String,
    pub deleted_user_name: String,
}

#[post("/users")]
async fn put_user(
    user_req: web::Json<AddUserRequest>,
    users: web::Data<Arc<RwLock<Users>>>,
    user_ctr: web::Data<Arc<RwLock<UserCounter>>>,
) -> Result<impl Responder, ServiceError> {
    if user_req.0.name.len() == 0 {
        return Err(ServiceError::BadRequest(String::from(
            "User name cannot be empty",
        )));
    }
    let users = &mut *users.write().unwrap();
    let mut user_ctr = &mut *user_ctr.write().unwrap();
    users.users.push(User {
        id: user_ctr.counter,
        name: user_req.name.clone(),
    });
    let response = AddUserResponse {
        result: String::from("registered"),
        user_id: user_ctr.counter,
    };
    user_ctr.counter += 1;

    Ok(Json(response))
}

#[get("/users")]
async fn get_users(
    users: web::Data<Arc<RwLock<Users>>>,
    _user_ctr: web::Data<Arc<RwLock<UserCounter>>>,
) -> Result<impl Responder, ServiceError> {
    let users = &*users.read().unwrap();

    Ok(Json(users.clone()))
}

#[derive(Deserialize)]
struct DeleteInfo {
    id: u32,
}

#[delete("/users/{id}")]
async fn delte_user(
    users: web::Data<Arc<RwLock<Users>>>,
    _user_ctr: web::Data<Arc<RwLock<UserCounter>>>,
    path: web::Path<DeleteInfo>,
) -> Result<impl Responder, ServiceError> {
    let users = &mut *users.write().unwrap();
    let id = path.id;
    let user = users.users.iter().enumerate().find(|x| x.1.id == id);
    let mut response = DeleteUserResponse {
        result: String::from("deleted"),
        deleted_user_name: String::from(""),
    };
    match user {
        None => {
            return Err(ServiceError::BadRequest(String::from(
                "No user with this ID found",
            )));
        }
        Some((ind, x)) => {
            response.deleted_user_name = x.name.clone();
            users.users.remove(ind);
        }
    }

    Ok(Json(response))
}
