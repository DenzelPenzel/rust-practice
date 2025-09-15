use async_recursion::*;
use std::future::Future;
use std::pin::Pin;

#[async_recursion]
async fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1).await + fibonacci(n - 2).await,
    }
}

async fn one() {
    println!("one");
}

async fn two() {
    println!("two");
}

async fn call(n: u32) -> Pin<Box<dyn Future<Output = ()>>> {
    match n {
        1 => Box::pin(one()),
        2 => Box::pin(two()),
        _ => panic!("Invalid number"),
    }
}

async fn run() {
    tokio::join!(one(), two());
}

#[tokio::main]
async fn main() {
    let x = fibonacci(10).await;
    println!("Fibonacci of 10 is {x}");

    let mut future = async {
        print!("Hello");
    };
    tokio::pin!(future);
    (&mut future).await;

    let (..) = tokio::join!(call(1), call(2));
}
