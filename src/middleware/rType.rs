use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub user_id: i32,
    pub email: String,
    pub exp: usize,
}