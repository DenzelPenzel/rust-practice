use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

pub fn greet_user(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:X}", hasher.finalize())
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

// pub fn get_users() -> Vec<User> {
//     vec![
//         User::new("admin", "admin", LoginRole::Admin),
//         User::new("ana", "123", LoginRole::User),
//     ]
// }

// fn get_admin_users() {
//     let users: Vec<String> = get_users()
//         .into_iter()
//         .filter(|u| u.role == LoginRole::Admin)
//         .map(|u| u.username)
//         .collect();
// }

pub fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        "admin".to_string(),
        User::new("admin", "admin", LoginRole::Admin),
    );
    users.insert("ana".to_string(), User::new("ana", "123", LoginRole::User));
    users
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load the file
        let users_json = std::fs::read_to_string(&users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

pub fn save_users(users: HashMap<String, User>) {
    let user_path = Path::new("users.json");
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(user_path, users_json).unwrap();
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();
    let password = hash_password(password);

    if let Some(user) = users.get(username) {
        return if user.password == password {
            Some(LoginAction::Granted(user.role.clone()))
        } else {
            Some(LoginAction::Denied)
        };
    }

    // if let Some(user) = users.iter().find(|u| u.username == username) {
    //     return if user.password == password {
    //         Some(LoginAction::Granted(user.role.clone()))
    //     } else {
    //         Some(LoginAction::Denied)
    //     };
    // }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("Hello, Jane!", greet_user("Jane"));
    }

    #[test]
    fn login_works() {
        assert_eq!(
            login("admin", "admin"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("ana", "123"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(login("wrong", "password"), None);
    }
}
