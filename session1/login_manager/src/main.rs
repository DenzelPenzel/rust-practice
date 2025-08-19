use auth;
use auth::{get_users, save_users, LoginRole, User};
use clap::builder::Str;
use clap::{Command, Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users
    List,
    /// Add a user.
    Add {
        /// User's login name
        username: String,
        /// User's password (plaintext)
        password: String,
        /// Optional - mark as an admin
        #[arg(long)]
        admin: Option<bool>,
    },
    /// Delete a user
    Delete {
        /// User to delete
        username: String,
    },
    /// Change a user's password
    ChangePassword {
        username: String,
        new_password: String,
    },
}

fn list_user() {
    println!("{:<20}{:<20}", "Username", "Password");
    println!("{:-<40}", "");

    let users = auth::get_users();
    users
        .iter()
        .for_each(|(_, user)| println!("{:<20}{:<20?}", user.username, user.role));
}

fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_users();
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };

    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(users);
}

fn delete_user(username: String) {
    let mut users = auth::get_users();
    if users.contains_key(&username) {
        users.remove(&username);
        save_users(users);
    } else {
        println!("User {} not found", username);
    }
}

fn change_password(username: String, new_password: String) {
    let mut users = auth::get_users();
    if users.contains_key(&username) {
        let user = users.get_mut(&username).unwrap();
        user.password = auth::hash_password(&new_password);
        save_users(users);
    } else {
        println!("User {} not found", username);
    }
}

fn main() {
    let cli = Args::parse();

    match cli.command {
        Some(Commands::List) => {
            list_user();
        }
        Some(Commands::Add {
            username,
            password,
            admin,
        }) => add_user(username, password, admin.unwrap_or(false)),
        Some(Commands::Delete { username }) => {
            delete_user(username);
        }
        Some(Commands::ChangePassword {
            username,
            new_password,
        }) => {
            change_password(username, new_password);
        }
        None => {
            println!("Run with --help for more info");
        }
    }
}
