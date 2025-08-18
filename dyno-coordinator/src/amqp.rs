use crate::config::Config;

pub async fn connect(config: &Config) -> String {
    println!("Connecting to: {}", config.amqp_uri);
    String::from("Connected")
}