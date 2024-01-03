use dotenv::dotenv;
use std::env;

pub fn load_env() {
    match env::var("SERVER_ENV") {
        Ok(enviroment) => {
            if enviroment == "DEV" {
                dotenv().ok();
            }
        }
        Err(_) => {
            println!("Environment not set, using default");
            dotenv().ok();
        }
    }
    println!("Environment: {}", env::var("SERVER_ENV").unwrap());
}
