use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct BalanceRequestPayload {
    pub id: u32,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<String>,
}

impl BalanceRequestPayload {
    fn new(id: u32, address: &String) -> Self {
        BalanceRequestPayload {
            id: id,
            jsonrpc: String::from("2.0"),
            method: String::from("eth_getBalance"),
            params: vec![address.to_string(), String::from("latest")],
        }
    }

    pub fn get_batch(addresses: Vec<String>) -> Vec<BalanceRequestPayload> {
        let mut payloads: Vec<BalanceRequestPayload> = vec![];

        for (id, address) in addresses.iter().enumerate() {
            payloads.push(BalanceRequestPayload::new(id as u32, address))
        }

        payloads
    }

    pub fn read_addresses(file_path: &str) -> Result<Vec<String>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let lines: Result<Vec<String>> = reader.lines().collect();
        lines
    }
}
