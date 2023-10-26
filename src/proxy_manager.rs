use reqwest::{Client, Error, Proxy};

pub fn get_proxy_url<'a>(proxy_url: &'a str) -> Option<&'a str> {
    if proxy_url.is_empty() {
        None
    } else {
        Some(proxy_url)
    }
}

pub fn init_client_with_proxy(proxy_url: Option<&str>) -> Result<Client, Error> {
    let mut client_builder = Client::builder();

    if let Some(proxy_url) = proxy_url {
        let proxy = Proxy::all(proxy_url)?;
        client_builder = client_builder.proxy(proxy);
    }

    client_builder.build().map_err(|e| e.into())
}
