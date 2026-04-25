//! User repository implementation.

use std::collections::HashMap;
use std::sync::RwLock;

use uuid::Uuid;

use crate::domain::model::user::User;
use crate::domain::port::user_repository_port::UserRepositoryPort;

pub struct UserRepository {
    users: RwLock<HashMap<Uuid, User>>,
    email_index: RwLock<HashMap<String, Uuid>>,
    username_index: RwLock<HashMap<String, Uuid>>,
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
            email_index: RwLock::new(HashMap::new()),
            username_index: RwLock::new(HashMap::new()),
        }
    }

    fn norm_email(email: &str) -> String {
        email.trim().to_ascii_lowercase()
    }

    fn norm_username(username: &str) -> String {
        username.trim().to_ascii_lowercase()
    }
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl UserRepositoryPort for UserRepository {
    fn save(&self, user: &User) -> bool {
        let email_key = Self::norm_email(&user.email);
        let username_key = Self::norm_username(&user.username);

        let mut users = match self.users.write() {
            Ok(v) => v,
            Err(_) => return false,
        };
        let mut email_index = match self.email_index.write() {
            Ok(v) => v,
            Err(_) => return false,
        };
        let mut username_index = match self.username_index.write() {
            Ok(v) => v,
            Err(_) => return false,
        };

        if let Some(existing) = email_index.get(&email_key) {
            if *existing != user.id {
                return false;
            }
        }

        if let Some(existing) = username_index.get(&username_key) {
            if *existing != user.id {
                return false;
            }
        }

        users.insert(user.id, user.clone());
        email_index.insert(email_key, user.id);
        username_index.insert(username_key, user.id);
        true
    }

    fn find_by_id(&self, id: Uuid) -> Option<User> {
        self.users.read().ok()?.get(&id).cloned()
    }

    fn find_by_email(&self, email: &str) -> Option<User> {
        let user_id = {
            let email_index = self.email_index.read().ok()?;
            email_index.get(&Self::norm_email(email)).copied()?
        };
        self.find_by_id(user_id)
    }

    fn find_by_username(&self, username: &str) -> Option<User> {
        let user_id = {
            let username_index = self.username_index.read().ok()?;
            username_index.get(&Self::norm_username(username)).copied()?
        };
        self.find_by_id(user_id)
    }
}
