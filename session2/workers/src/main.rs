use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command {
    Run(Job),
    Quit,
}

fn hi_there() {
    println!("Hi there!");
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Run(job) => job(),
                Command::Quit => break,
            }
        }
    });

    /*
    let job = || println!("Hello, world!");
    let job2 = || {
        for i in 0..10 {
            println!("{i}");
        }
    };


    tx.send(Box::new(job)).unwrap();
    tx.send(Box::new(job2)).unwrap();
    tx.send(Box::new(hi_there)).unwrap();
    */

    tx.send(Command::Run(Box::new(|| println!("New Box"))))
        .unwrap();
    tx.send(Command::Quit).unwrap();

    handle.join().unwrap();
}
