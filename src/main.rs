use starknet::{providers::Provider, macros::selector};
use starknet::core::types::{FieldElement, BroadcastedInvokeTransactionV1, BroadcastedInvokeTransaction};
use eyre::Result;
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use url::Url;
use starknet::accounts::Call;
use hex;


type Error = String;

pub const EXECUTE: FieldElement = selector!("execute");


pub struct KakarotClient<StarknetClient>
where
    StarknetClient: Provider,
{
    starknet_provider: StarknetClient,
    kakarot_address: FieldElement,
    proxy_account_class_hash: FieldElement,
}

impl KakarotClient<JsonRpcClient<HttpTransport>>{


    pub fn new(starknet_config: StarknetConfig) -> Result<Self> {
        let StarknetConfig { starknet_rpc, kakarot_address, proxy_account_class_hash } = starknet_config;
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
    async fn call_execute(&self, value : FieldElement, bytecode: Vec<FieldElement>, kakarot_calldata: Vec<FieldElement> ){
        let starknet_contract_address = self.proxy_account_class_hash;
        let evm_contract_address = self.kakarot_address;
        let kakarot_calldata_len = FieldElement::from(kakarot_calldata.len());
        let bytecode_len = FieldElement::from(bytecode.len());
        let zero:u8 = 0;
        let gas_limit = FieldElement::from(zero);
        let gas_price = FieldElement::from(zero);

        // Create execute_arguments calldata

        let mut execute_arguments = vec![];
        execute_arguments.push(starknet_contract_address); //starknet_contract_address
        execute_arguments.push(evm_contract_address); //evm_contract_address
        execute_arguments.push(bytecode_len); //bytecode_len
        for bytecode_bytes in bytecode { //bytecode
            execute_arguments.push(bytecode_bytes);
        }
        execute_arguments.push(kakarot_calldata_len); //kakarot_calldata_len
        for calldata_bytes in kakarot_calldata { //kakarot_calldata
            execute_arguments.push(calldata_bytes);
        }
        execute_arguments.push(value); //value
        execute_arguments.push(gas_limit); //gas_limit
        execute_arguments.push(gas_price); //gas_price

        let raw_calldata = raw_starknet_calldata(self.kakarot_address, execute_arguments);

        // println!("raw_calldata: {:?}", raw_calldata);

        //TODO: Get Starknet Address
        let starknet_address = FieldElement::from(3_u8);

        println!("starknet_address: {:?}", starknet_address);

        //TODO: Get Nonce
        let nonce = FieldElement::ZERO;

        // Get estimated_fee from Starknet
        let max_fee = FieldElement::MAX;

        let signature = vec![];

        let request =
            BroadcastedInvokeTransactionV1 { max_fee, signature, nonce, sender_address: starknet_address, calldata : raw_calldata };

        // print!("request: {:?}", request);

        let response = self.starknet_provider.add_invoke_transaction(&BroadcastedInvokeTransaction::V1(request)).await.unwrap();
        println!("response: {:?}", response);
    }
}


fn get_env_var(name: &str) -> Result<String, Error> {
    std::env::var(name).map_err(|_| "Environment variable not found".to_string())
}


#[derive(Debug, Clone)]
pub struct StarknetConfig {
    pub starknet_rpc: String,
    pub kakarot_address: FieldElement,
    pub proxy_account_class_hash: FieldElement,
}

impl StarknetConfig {
    pub fn new(starknet_rpc: &str, kakarot_address: FieldElement, proxy_account_class_hash: FieldElement) -> Self {
        StarknetConfig { starknet_rpc: String::from(starknet_rpc), kakarot_address, proxy_account_class_hash }
    }

    pub fn from_env() -> Result<Self, Error> {
        let starknet_rpc_url = get_env_var("STARKNET_RPC_URL")?;

        let kakarot_address = get_env_var("KAKAROT_ADDRESS")?;
        let kakarot_address = FieldElement::from_hex_be(&kakarot_address).map_err(|_| {
            format!(
                "KAKAROT_ADDRESS should be provided as a hex string, got {kakarot_address}"
            )
        })?;

        let proxy_account_class_hash = get_env_var("PROXY_ACCOUNT_CLASS_HASH")?;
        let proxy_account_class_hash = FieldElement::from_hex_be(&proxy_account_class_hash).map_err(|_| {
            format!(
                "PROXY_ACCOUNT_CLASS_HASH should be provided as a hex string, got {proxy_account_class_hash}"
            )
        })?;

        Ok(StarknetConfig::new(&starknet_rpc_url, kakarot_address, proxy_account_class_hash))
    }
}


pub fn raw_starknet_calldata(kakarot_address: FieldElement, execute_arguments: Vec<FieldElement>) -> Vec<FieldElement> {
    let calls: Vec<Call> =
        vec![Call { to: kakarot_address, selector: EXECUTE, calldata: execute_arguments }];
    let mut concated_calldata: Vec<FieldElement> = vec![];
    let mut execute_calldata: Vec<FieldElement> = vec![calls.len().into()];
    for call in &calls {
        execute_calldata.push(call.to); // to
        execute_calldata.push(call.selector); // selector
        execute_calldata.push(concated_calldata.len().into()); // data_offset
        execute_calldata.push(call.calldata.len().into()); // data_len

        for item in &call.calldata {
            concated_calldata.push(*item);
        }
    }
    execute_calldata.push(concated_calldata.len().into()); // calldata_len
    for item in concated_calldata {
        execute_calldata.push(item); // calldata
    }

    execute_calldata
}


fn main() {
    dotenv::dotenv().ok();
    let starknet_config = StarknetConfig::from_env().unwrap();

    let kakarot_client = KakarotClient::new(starknet_config.clone()).unwrap();

    let value = FieldElement::from(0_u8);
    let code = "604260005260206000F3";
    let bytecode = hex_to_field_elements(code);
    let kakarot_calldata = vec![];

    // Here is how you call the function, wrapped in an async block to handle the async nature of call_execute
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        kakarot_client.call_execute(value, bytecode, kakarot_calldata).await;
    });

    println!("starknet_config: {:?}", starknet_config);
}



fn hex_to_field_elements(hex: &str) -> Vec<FieldElement> {
    // Convert the hex string to bytes
    let bytes = hex::decode(hex).expect("Invalid hex string");

    // Convert the bytes to FieldElement
    let elements: Vec<FieldElement> = bytes.iter().map(|byte| FieldElement::from(*byte)).collect();

    elements
}
