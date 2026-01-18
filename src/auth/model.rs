pub struct User {
    pub nome: String
}

impl User {
    pub fn create(nome: &str) -> User {
        User { nome: String::from(nome) }
    }
}
