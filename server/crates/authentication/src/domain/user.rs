use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub pseudo: Option<String>,
}

impl User {
    pub fn new(id: Uuid, email: String, password: String, pseudo: Option<String>) -> Self {
        Self {
            id,
            email,
            password,
            pseudo,
        }
    }
}
