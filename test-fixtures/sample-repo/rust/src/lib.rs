use crate::repository::UserRepository;

pub struct UserService {
    repository: UserRepository,
}

pub trait UserReader {
    fn find_user(&self, id: &str) -> Option<String>;
}

impl UserReader for UserService {
    fn find_user(&self, id: &str) -> Option<String> {
        self.repository.find(id)
    }
}

pub mod repository {
    pub struct UserRepository;

    impl UserRepository {
        pub fn find(&self, id: &str) -> Option<String> {
            Some(id.to_string())
        }
    }
}

