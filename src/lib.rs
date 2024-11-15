//!
//! Stylus Match Information Contract
//! 
//! This contract is used to store match related information

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*, storage::{StorageAddress, StorageBool, StorageU256}};

#[storage]
#[entrypoint]
pub struct MatchInformationContract{
    initialized: StorageBool,
    owner: StorageAddress,
    matchmaking_server_wallet_address: StorageAddress,
    player_info_smart_contract_address: StorageAddress,
    prediction_smart_contract_address: StorageAddress,
    latest_match_id: StorageU256,
}

#[public]
impl MatchInformationContract{
    fn init(&mut self) -> Result<(), Vec<u8>>{
        let initialized = self.initialized.get();
        if initialized {
            return Err("Already initialized".into());
        }
        self.owner.set(msg::sender());
        Ok(())
    }
}
