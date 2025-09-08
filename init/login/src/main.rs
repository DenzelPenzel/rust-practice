use auth::{login, read_line, LoginAction, LoginRole};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter username:");
        let username = read_line();
        println!("Enter password:");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => println!("Admin logged in"),
                    LoginRole::User => println!("User logged in"),
                }
                break;
            }
            Some(LoginAction::Denied) => {
                // Do nothing
            }
            None => {
                println!("Invalid username or password");
            }
        }

        print!("Invalid username:");
        tries += 1;
        if tries > 5 {
            println!("Too many errors!");
            break;
        }
    }
}
