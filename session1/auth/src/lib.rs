pub fn greet_user(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password = password.to_lowercase();

    if username != "admin" && username != "ana" {
        return None;
    }

    if username == "admin" && password == "admin" {
        Some(LoginAction::Granted(LoginRole::Admin))
    } else if username == "ana" && password == "123" {
        Some(LoginAction::Granted(LoginRole::User))
    } else {
        Some(LoginAction::Denied)
    }
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
        assert_eq!(login("admin", "admin"), LoginAction::Admin);
        assert_eq!(login("ana", "123"), LoginAction::User);
    }
}
