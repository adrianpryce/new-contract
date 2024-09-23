use near_sdk::{
    near_bindgen,
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    log,
};
use schemars::JsonSchema;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contract_metadata: ContractSourceMetadata,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema, Clone, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractSourceMetadata {
    version: Option<String>,
    link: Option<String>,
    standards: Option<Vec<Standard>>,
    build_info: Option<BuildInfo>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Standard {
    standard: String,
    version: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BuildInfo {
    build_environment: String,
    source_code_snapshot: String,
    contract_path: Option<String>,
    build_command: Vec<String>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(metadata: ContractSourceMetadata) -> Self {
        Self {
            contract_metadata: metadata,
        }
    }

    // Getter method to return contract metadata, exposing NEP-330 reproducible build information
    pub fn get_metadata(&self) -> ContractSourceMetadata {
        self.contract_metadata.clone()
    }

    // Helper method to log NEP-330 build information, useful for ecosystem tools.
    pub fn log_metadata(&self) {
        log!("Build Environment: {}", self.contract_metadata.build_info.as_ref().unwrap().build_environment);
        log!("Source Code Snapshot: {}", self.contract_metadata.build_info.as_ref().unwrap().source_code_snapshot);
    }
}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // Add tests as needed for NEP-330 reproducibility metadata
    }
}

