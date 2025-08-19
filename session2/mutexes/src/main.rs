use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::sync::RwLock;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(vec![]);

fn lock_numbers() {
    let mut handlers = Vec::new();

    for _ in 0..10 {
        let hanlde = std::thread::spawn(|| {
            let mut lock = NUMBERS.lock().unwrap();
            lock.push(1);
        });
        handlers.push(hanlde);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());
    let lock = NUMBERS.lock().unwrap();
    println!("Numbers: {:#?}", lock);
}

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    vec!["Ana".to_string(), "Jane".to_string()]
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    lock_numbers();

    std::thread::spawn(|| {
        loop {
            println!("Reading users...");
            let users = USERS.read().unwrap();
            println!("{users:?}");
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    loop {
        println!("Enter a new user:");
        let input = read_line();
        if input == "q" {
            break;
        } else {
            let mut lock = USERS.write().unwrap();
            lock.push(input);
        }
    }
}
