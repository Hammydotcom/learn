use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let mut seconds = 1;
        loop {
            println!("Seconds: {}", seconds);
            seconds += 1;
            sleep(Duration::from_secs(1)).await; 
        }
    });
    loop {}
}
