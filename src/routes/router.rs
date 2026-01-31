#[derive(Clone)]
pub struct Router {
    pub base_path: String,
}

impl Router {
    pub fn assets(&self, path: &str) -> String {
        if self.base_path.is_empty() || self.base_path == "/" {
            format!("/{}", path)
        } else {
            format!("{}/{}", self.base_path.trim_end_matches('/'), path)
        }
    }

    pub fn root_path(&self) -> String {
        format!("{}/", &self.base_path)
    }

    // BOOKS ROUTES

    pub fn new_book_path(&self) -> String {
        format!("{}/books/new", &self.base_path)
    }

    pub fn create_book_path(&self) -> String {
        format!("{}/books", &self.base_path)
    }

    pub fn update_book_path(&self, id: &i32) -> String {
        format!("{}/books/{}", &self.base_path, id)
    }

    pub fn download_csv_book_path(&self) -> String {
        format!("{}/books/download_csv", &self.base_path)
    }

    // USERS

    pub fn index_user_path(&self) -> String {
        format!("{}/users", &self.base_path)
    }

    pub fn new_user_path(&self) -> String {
        format!("{}/users/new", &self.base_path)
    }

    pub fn create_user_path(&self) -> String {
        format!("{}/users", &self.base_path)
    }

    pub fn update_user_path(&self, id: &i32) -> String {
        format!("{}/users/{}", &self.base_path, id)
    }
}
