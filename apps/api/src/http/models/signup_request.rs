
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupRequest {
    pub email: String,
    pub innie_name: String,
    pub password: String,
}