use std::time::Duration;
use tokio::time::sleep;

pub async fn hello() {
    println!("Hello async Rust!");
}

pub async fn add(x: u32, y: u32) -> u32 {
    sleep(Duration::from_millis(1000)).await;
    x + y
}

#[tokio::main]

pub async fn main() {

    hello().await;

    let sum = add(1,2).await;
        
    println!("sum: {sum}");
}
