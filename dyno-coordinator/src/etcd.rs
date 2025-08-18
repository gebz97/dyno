use etcd_client::{Client, Error};

use crate::config::Config;

pub async fn connect(config: &Config) -> Result<Client, Error> {
    let options = config.get_etcd_options(); // returns ConnectOptions
    let client = etcd_client::Client::connect([config.etcd_uri.as_str()], Some(options)).await?;
    Ok(client)
}