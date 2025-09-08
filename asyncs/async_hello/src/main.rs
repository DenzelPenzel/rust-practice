async fn fn1() -> i32 {
    1
}

async fn fn2() -> i32 {
    2
}

async fn fn3() {
    for i in 0..10 {
        println!("tick {i}");
        // yield the current task to the scheduler
        tokio::task::yield_now().await;
    }
}

#[tokio::main()]
async fn main() {
    let (res1, res2) = tokio::join!(fn1(), fn2());
    println!("res1: {:?}, res2: {:?}", res1, res2);


    println!("================");

    let fn2_handle = tokio::spawn(fn2()); 
    fn2_handle.await.unwrap();

    println!("================");

    let _ = tokio::join!(
        tokio::spawn(fn3()),
        tokio::spawn(fn3()),
    );

    println!("Done")
}
