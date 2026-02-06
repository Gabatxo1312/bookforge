use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ApiConfig {
    pub google_books_api_key: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            google_books_api_key: "".to_string(),
        }
    }
}
