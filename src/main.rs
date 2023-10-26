mod balance_request;
mod config;
mod proxy_manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proxy_url = proxy_manager::get_proxy_url(config::PROXY_URL);
    let client = proxy_manager::init_client_with_proxy(proxy_url)?;

    let addresses =
        balance_request::BalanceRequestPayload::read_addresses(config::ADDRESSES_FILE_PATH)?;
    let payloads = balance_request::BalanceRequestPayload::get_batch(addresses);
    let response = client
        .post(config::RPC_URL)
        .json(&payloads)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    print!("{:#?}", response);

    Ok(())
}
