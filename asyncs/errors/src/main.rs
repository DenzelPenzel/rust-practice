use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
enum UserError {
    #[error("File not found")]
    FileNotFound,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("No users")]
    NoUsers,
    #[error("Too many users")]
    TooManyUsers,
    #[error("Other error: {0}")]
    Other(std::io::Error),
}

fn maybe_read() -> Result<String, std::io::Error> {
    let f = Path::new("myfile.txt");
    std::fs::read_to_string(f)
}

fn file_to_upper() -> Result<String, std::io::Error> {
    let contents = maybe_read()?;
    Ok(contents.to_uppercase())
}

#[derive(Deserialize)]
struct User {
    user: String,
}

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// fn load_user() -> anyhow::Result<Vec<User>> {
//     let f = Path::new("user.json");
//     let raw_data = std::fs::read_to_string(f)?;
//     let users = serde_json::from_str(&raw_data)?;
//     Ok(users)
// }

fn load_user() -> Result<Vec<User>, UserError> {
    let f = Path::new("user.json");
    let raw_data = std::fs::read_to_string(f).map_err(|_| UserError::NoUsers)?;
    let users = serde_json::from_str(&raw_data).map_err(|_| UserError::NoUsers)?;
    Ok(users)
}

fn main() {
    let users = load_user();
    match users {
        Ok(users) => println!("Users: {:?}", users.len()),
        Err(e) => println!("Error: {:?}", e),
    }

    if let Ok(content) = file_to_upper() {
        println!("Content: {}", content);
    }

    let f = Path::new("myfile.txt");
    let content = std::fs::read_to_string(f);
    match content {
        Ok(content) => println!("Content: {}", content),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found"),
            std::io::ErrorKind::PermissionDenied => println!("Permission denied"),
            _ => println!("Other error: {}", e),
        },
    }
}
