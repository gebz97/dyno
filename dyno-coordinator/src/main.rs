use anyhow::{Context, Error};

use crate::registry::Registry;

mod amqp;
mod cli;
mod config;
mod etcd;
mod registry;
mod scheduler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 1. Load configuration
    let config = cli::init()
        .context("Unable to initialize Configuration")
        .expect("Invalid arguments");

    // 2. Connect to Etcd
    let mut _etcd_client = etcd::connect(&config)
        .await
        .context("Failed to connect to Etcd")?;

    // 3. Connect to AMQP
    let _amqp_client = amqp::connect(&config)
        .await;
        //.await
        //.context("Failed to connect to AMQP")?;

    // 4. Instantiate Registry
    let _registry_handle = Registry::init();

    // println!("All dependencies initialized. Starting scheduler...");

    // // 5. Start the scheduler (infinite loop)
    // scheduler::start(&config, &etcd_client, &amqp_client, &registry_handle).await?;

    // Normally never reached unless scheduler exits
    Ok(())
}
