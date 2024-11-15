//!
//! Stylus Match Information Contract
//! 
//! This contract is used to store match related information

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::Address;
use alloy_sol_types::sol;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*, storage::{StorageAddress, StorageBool, StorageU256}};

sol!{
    event matchCreated(uint256 indexed match_id, address player1, string signature);
    event matchJoined(uint256 indexed match_id, address player2, string signature);
    event matchStarted(uint256 indexed match_id, address player1, address player2, string signature);
    event matchEnded(uint256 indexed match_id, address winner, string signature);
}

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
        self.latest_match_id.set(U256::from(0));
        Ok(())
    }
    
    //view current match making server wallet
    fn get_matchmaking_server_wallet_address(&self) -> Address{
        self.matchmaking_server_wallet_address.get()
    }

    //view current player info smart contract address
    fn get_player_info_smart_contract_address(&self) -> Address{
        self.player_info_smart_contract_address.get()
    }

    //view current prediction smart contract address
    fn get_prediction_smart_contract_address(&self) -> Address{
        self.prediction_smart_contract_address.get()
    }

    //only owner can set match_making_address
    fn set_matchmaking_server_wallet_address(&mut self, address: Address) -> Result<(), Vec<u8>>{
        if self.owner.get() != msg::sender(){
            return Err("Only owner can set match_making_address".into());
        }
        self.matchmaking_server_wallet_address.set(address);
        Ok(())
    }

    //only owner can set player_info_smart_contract_address
    fn set_player_info_smart_contract_address(&mut self, address: Address) -> Result<(), Vec<u8>>{
        if self.owner.get() != msg::sender(){
            return Err("Only owner can set player_info_smart_contract_address".into());
        }
        self.player_info_smart_contract_address.set(address);
        Ok(())
    }

    //only owner can set prediction_smart_contract_address
    fn set_prediction_smart_contract_address(&mut self, address: Address) -> Result<(), Vec<u8>>{
        if self.owner.get() != msg::sender(){
            return Err("Only owner can set prediction_smart_contract_address".into());
        }
        self.prediction_smart_contract_address.set(address);
        Ok(())
    }
    
    //get the latest match id
    fn get_latest_match_id(&self) -> U256{
        self.latest_match_id.get()
    }

    
}
