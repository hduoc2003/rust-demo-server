use axum::{routing::{get, post}, Router};

use crate::v1::api::{controllers::signup::handle_signup, db::database::Pool};

pub fn routes() -> Router<Pool> {
    Router::new().route("/signup", post(handle_signup))
}
