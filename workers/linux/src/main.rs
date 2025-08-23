use std::error::Error;

use serde::{Deserialize, Serialize};
// use ssh2;
use vaultrs::{client, kv2};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let vault_client = client::VaultClient::new(
        client::VaultClientSettingsBuilder::default()
            .address("http://sandbox.dyno.gebz.local:8200")
            .token("s.RupIuxHDMXI3DnKORlrJNkh8")
            .verify(false)
            .build()
            .unwrap(),
    )
    .unwrap();

    let example_cred = HostCredentials {
        remote_user: String::from("dummy"),
        remote_password: String::from("dummy123"),
        remote_sudo_password: String::from("dummy123"),
    };

    let res = kv2::set(
        &vault_client,
        "dyno_credentials",
        "inventories/1/hosts/ssh/2",
        &example_cred,
    ).await?;

    println!("{:?}", res);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct HostCredentials {
    remote_user: String,
    remote_password: String,
    remote_sudo_password: String,
}
