use coin_watcher::config::*;
use iota_client::{
    Client, Result,
};

#[tokio::main]
pub async fn main() -> Result<()> {

    Config::read_file().await;

    // let node_url = "https://chrysalis-nodes.iota.org";

    // let iota = Client::builder()
    //     .with_node("https://chrysalis-nodes.iota.org")?
    //     .finish()
    //     .await?;

    // let info = iota.get_info().await?;

    // let address = ["iota1qrmmva42tzhy9262rtqnf9spphn3vucj0ajrpfzfykue5xj4v3gg2zu58jg ".to_string()];

    // let balance = iota.get_address_balances(&address).await?;

    // println!("{balance:?}");

    Ok(())
}
