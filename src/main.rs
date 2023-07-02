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
    value: u128,
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
    let file = match File::open("test_cases.json") {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open test_cases.json: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);

    let test_cases: Vec<TestCase> = match serde_json::from_reader(reader) {
        Ok(cases) => cases,
        Err(e) => {
            println!("Failed to parse test cases: {}", e);
            return;
        }
    };

    for (index, test_case) in test_cases.iter().enumerate() {
        println!("Running test case {}: {}", index + 1, test_case.id);

        let value = FieldElement::from(test_case.params.value);
        let code = &test_case.params.code;
        let calldata = &test_case.params.calldata;
        let bytecode = hex_to_field_elements(code);
        let kakarot_calldata = hex_to_field_elements(calldata);

        let result = tokio::runtime::Runtime::new().unwrap().block_on(async {
            kakarot_client
                .call_execute(value, bytecode, kakarot_calldata)
                .await
        });

        match result {
            Ok(_) => println!("Test case {} finished ✅", test_case.id),
            Err(e) => println!("Test case {} failed: {} ❌", test_case.id, e),
        };
    }
}
