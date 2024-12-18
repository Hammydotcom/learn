use tokio::time::{Duration, interval};
use tokio::io::{self, AsyncBufReadExt};
#[tokio::main]
async fn main() {
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();
    let mut seconds = 0;
    let mut timer = interval(Duration::from_secs(1));
    loop {
        tokio::select! {
            _ = timer.tick() => {
                println!("\nSeconds: {}", seconds);
                seconds += 1;
            }
            result = reader.read_line(&mut input) => {
                if let Ok(bytes_read) = result {
                    if bytes_read > 0 {
                        println!("You typed: {}", input.trim());
                        input.clear();
                    }
                } else {
                    eprintln!("Error reading input.");
                }
            }
        }
    }
}
