use redis::AsyncCommands;
use tokio::time::{Duration, interval};
use tokio::io::{self, AsyncBufReadExt};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() 
{
    let client = redis::Client::open("redis://127.0.0.1:6379/").expect("Failed to create Redis client");
    let con = Arc::new(Mutex::new(client.get_multiplexed_async_connection().await.expect("Failed to connect to Redis")));
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();
    let mut seconds = 0;
    let mut timer = interval(Duration::from_secs(1));
    println!("Program started! Type something and press Enter (type 'exit' to quit):");

    loop 
    {
        tokio::select! 
        {
            _ = timer.tick() => 
            {
                println!("\nSeconds: {}", seconds);
                let mut con = con.lock().await;
                let _: () = con.set("elapsed_seconds", seconds).await.expect("Failed to set seconds in Redis");
                seconds += 1;
            }
            result = reader.read_line(&mut input) => 
            {
                if let Ok(bytes_read) = result 
                {
                    if bytes_read > 0 
                    {
                        let trimmed_input = input.trim();
                        if trimmed_input.eq_ignore_ascii_case("exit") 
                        {
                            println!("Exiting program...");
                            break;
                        }
                        println!("You typed: {}", trimmed_input);
                        let mut con = con.lock().await;     
                        match trimmed_input 
                        {
                            "hello" => 
                            {
                                let _: () = con.set("response", "Hi there!").await.expect("Failed to set response in Redis");
                            },
                            "bye" => 
                            {
                                let _: () = con.set("response", "Goodbye!").await.expect("Failed to set response in Redis");
                            },
                            _ => 
                            {
                                let _: () = con.set("response", "I don't understand that command.").await.expect("Failed to set response in Redis");
                            },
                        }
                        let key = format!("input:{}", seconds);
                        let _: () = con.set(key, trimmed_input).await.expect("Failed to set input in Redis");
                        input.clear();
                    }
                } else 
                {
                    eprintln!("Error reading input.");
                }
            }
        }
    }
    println!("Program terminated.");
}
