use super::config::StarknetConfig;
use eyre::Result;
use starknet::core::types::{BlockId, BlockTag, FieldElement, FunctionCall};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::{macros::selector, providers::Provider};
use std::error::Error;
use url::Url;

// pub type Error = String;

pub const EXECUTE: FieldElement = selector!("execute");
pub const CHECK_VALUE: FieldElement = selector!("check_value");

pub struct KakarotClient<StarknetClient>
where
    StarknetClient: Provider,
{
    starknet_provider: StarknetClient,
    kakarot_address: FieldElement,
    proxy_account_class_hash: FieldElement,
}

impl KakarotClient<JsonRpcClient<HttpTransport>> {
    pub fn new(starknet_config: StarknetConfig) -> Result<Self> {
        let StarknetConfig {
            starknet_rpc,
            kakarot_address,
            proxy_account_class_hash,
        } = starknet_config;
        let url = Url::parse(&starknet_rpc)?;
        Ok(Self {
            starknet_provider: JsonRpcClient::new(HttpTransport::new(url)),
            kakarot_address,
            proxy_account_class_hash,
        })
    }

    // Calling function execute
    // func execute{
    //     syscall_ptr: felt*,
    //     pedersen_ptr: HashBuiltin*,
    //     range_check_ptr,
    //     bitwise_ptr: BitwiseBuiltin*,
    // }(
    //     starknet_contract_address: felt,
    //     evm_contract_address: felt,
    //     bytecode_len: felt,
    //     bytecode: felt*,
    //     calldata_len: felt,
    //     calldata: felt*,
    //     value: felt,
    //     gas_limit: felt,
    //     gas_price: felt,
    // ) ->
    pub async fn call_execute(
        &self,
        value: FieldElement,
        bytecode: Vec<FieldElement>,
        kakarot_calldata: Vec<FieldElement>,
    ) -> Result<(), Box<dyn Error>> {
        let starknet_contract_address = FieldElement::from(0_u8);
        let evm_contract_address = FieldElement::from(0_u8);
        let kakarot_calldata_len = FieldElement::from(kakarot_calldata.len());
        let bytecode_len = FieldElement::from(bytecode.len());
        let zero: u8 = 0;
        let gas_limit = FieldElement::from(zero);
        let gas_price = FieldElement::from(zero);

        // Create execute_arguments calldata

        let mut execute_arguments = vec![];
        execute_arguments.push(starknet_contract_address); //starknet_contract_address
        execute_arguments.push(evm_contract_address); //evm_contract_address
        execute_arguments.push(bytecode_len); //bytecode_len
        for bytecode_bytes in bytecode {
            //bytecode
            execute_arguments.push(bytecode_bytes);
        }
        execute_arguments.push(kakarot_calldata_len); //kakarot_calldata_len
        for calldata_bytes in kakarot_calldata {
            //kakarot_calldata
            execute_arguments.push(calldata_bytes);
        }
        execute_arguments.push(value); //value
        execute_arguments.push(gas_limit); //gas_limit
        execute_arguments.push(gas_price); //gas_price

        let starknet_block_id = BlockId::Tag(BlockTag::Latest);

        let request = FunctionCall {
            contract_address: self.kakarot_address,
            entry_point_selector: EXECUTE,
            calldata: execute_arguments,
        };

        let call_result: Vec<FieldElement> = self
            .starknet_provider
            .call(request, starknet_block_id)
            .await?;

        Ok(())
    }

    pub async fn call_check_value(&self) {
        let execute_arguments = vec![];

        let starknet_block_id = BlockId::Tag(BlockTag::Latest);

        let request = FunctionCall {
            contract_address: self.kakarot_address,
            entry_point_selector: CHECK_VALUE,
            calldata: execute_arguments,
        };

        let call_result: Vec<FieldElement> = self
            .starknet_provider
            .call(request, starknet_block_id)
            .await
            .unwrap();
        // println!("response: {:?}", call_result);
    }
}
