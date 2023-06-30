mod client;
mod config;
mod utils;

use client::KakarotClient;
use config::StarknetConfig;
use dotenv::dotenv;
use starknet::core::types::FieldElement;
use utils::{hex_to_field_elements, Error};

use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct TestCase {
    params: Params,
    id: String,
    marks: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Params {
    value: u8,
    code: String,
    calldata: String,
    stack: String,
    memory: String,
    return_value: String,
}

fn main() {
    dotenv().ok();
    let starknet_config = StarknetConfig::from_env().unwrap();
    let kakarot_client = KakarotClient::new(starknet_config.clone()).unwrap();

    // Load test cases from json file
    let file = File::open("test_cases.json").unwrap();
    let reader = BufReader::new(file);

    let test_cases: Vec<TestCase> = serde_json::from_reader(reader).unwrap();

    for (index, test_case) in test_cases.iter().enumerate() {
        println!("Running test case {}: {}", index + 1, test_case.id);

        let value = FieldElement::from(test_case.params.value);
        let code = &test_case.params.code;
        let bytecode = hex_to_field_elements(code);
        let kakarot_calldata = vec![];

        tokio::runtime::Runtime::new().unwrap().block_on(async {
            kakarot_client
                .call_execute(value, bytecode, kakarot_calldata)
                .await;

            kakarot_client.call_check_value().await;
        });

        println!("Test case {} finished", test_case.id);
    }
}
