use hex;
use starknet::core::types::FieldElement;

pub type Error = String;

pub fn hex_to_field_elements(hex: &str) -> Vec<FieldElement> {
    // Convert the hex string to bytes
    let bytes = hex::decode(hex).expect("Invalid hex string");

    // Convert the bytes to FieldElement
    let elements: Vec<FieldElement> = bytes.iter().map(|byte| FieldElement::from(*byte)).collect();

    elements
}
