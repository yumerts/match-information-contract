//!
//! Stylus Match Information Contract
//! 
//! This contract is used to store match related information

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use std::str::FromStr;

use alloy_primitives::{Address, Signature};
use alloy_sol_types::sol;
// use ethers::{core::k256::ecdsa::hazmat::verify_prehashed, etherscan::verify};
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*, storage::{StorageAddress, StorageBool, StorageMap, StorageU256, StorageU8}};

sol!{
    event matchCreated(uint256 indexed match_id, address player1, string signature);
    event matchJoined(uint256 indexed match_id, address player2, string signature);
    event matchStarted(uint256 indexed match_id, address player1, address player2, string signature);
    event matchEnded(uint256 indexed match_id, address winner, string signature);
}


enum GameState{
    Finding,
    Matched,
    Started,
    Ended
}

#[storage]
struct Match{
    exists: StorageBool,
    player1: StorageAddress,
    player2: StorageAddress,
    state: StorageU8 //0 for Finding, 1 for Matched (Ready for Prediction), 2 for Started, 3 for Ended
}

#[storage]
#[entrypoint]
struct MatchInformationContract{
    initialized: StorageBool,
    owner: StorageAddress,
    matchmaking_server_wallet_address: StorageAddress,
    player_info_smart_contract_address: StorageAddress,
    prediction_smart_contract_address: StorageAddress,
    latest_match_id: StorageU256,
    matches: StorageMap<U256, Match>
}

#[public]
impl MatchInformationContract{
    pub fn init(&mut self) -> Result<(), Vec<u8>>{
        let initialized = self.initialized.get();
        if initialized {
            return Err("Already initialized".into());
        }
        self.owner.set(msg::sender());
        self.latest_match_id.set(U256::from(0));
        Ok(())
    }
    
    //view current match making server wallet
    pub fn get_matchmaking_server_wallet_address(&self) -> Address{
        self.matchmaking_server_wallet_address.get()
    }

    //view current player info smart contract address
    pub fn get_player_info_smart_contract_address(&self) -> Address{
        self.player_info_smart_contract_address.get()
    }

    //view current prediction smart contract address
    pub fn get_prediction_smart_contract_address(&self) -> Address{
        self.prediction_smart_contract_address.get()
    }

    //only owner can set match_making_address
    pub fn set_matchmaking_server_wallet_address(&mut self, address: Address) -> Result<(), Vec<u8>>{
        if !self.initialized.get() {
            return Err("has not been initialized yet".into());
        }
        if self.owner.get() != msg::sender(){
            return Err("Only owner can set match_making_address".into());
        }
        self.matchmaking_server_wallet_address.set(address);
        Ok(())
    }

    //only owner can set player_info_smart_contract_address
    pub fn set_player_info_smart_contract_address(&mut self, address: Address) -> Result<(), Vec<u8>>{
        if !self.initialized.get() {
            return Err("has not been initialized yet".into());
        }

        if self.owner.get() != msg::sender(){
            return Err("Only owner can set player_info_smart_contract_address".into());
        }
        self.player_info_smart_contract_address.set(address);
        Ok(())
    }

    //only owner can set prediction_smart_contract_address
    pub fn set_prediction_smart_contract_address(&mut self, address: Address) -> Result<(), Vec<u8>>{
        if !self.initialized.get() {
            return Err("has not been initialized yet".into());
        }

        if self.owner.get() != msg::sender(){
            return Err("Only owner can set prediction_smart_contract_address".into());
        }
        self.prediction_smart_contract_address.set(address);
        Ok(())
    }
    
    //get the latest match id
    pub fn get_latest_match_id(&self) -> Result<U256, Vec<u8>>{

        if !self.initialized.get() {
            return Err("has not been initialized yet".into());
        }

        Ok(self.latest_match_id.get())
    }

    //create a new match
    pub fn create_match(&mut self, player1: Address, server_signature_string: String) -> Result<(), Vec<u8>>{
       /*
        if !self.initialized.get() {
            return Err("has not been initialized yet".into());
        }

        let latest_match_id = self.latest_match_id.get();
        //checks if the digit
        alloy_primitives::Signature::

        //add match to the storage
        let mut match_setter = self.matches.setter(latest_match_id);
        match_setter.exists.set(true);
        match_setter.player1.set(player1);

        let match_id = self.latest_match_id.get() + U256::from(1);
        self.latest_match_id.set(match_id);
        
        Ok(())*/
        Ok(())
    }

    //join a match
    pub fn join_match(&mut self, player2: Address, server_signature_string: String) -> Result<(), Vec<u8>>{
        Ok(())
    }

    //start a match (only allowed for the match making server)
    pub fn start_match(&mut self, match_id: U256) -> Result<(), Vec<u8>>{
        if !self.initialized.get() {
            return Err("has not been initialized yet".into());
        }

        let current_match = self.matches.get(match_id);
        if !current_match.exists.get(){
            return Err("Match does not exist".into());
        }

        if self.matchmaking_server_wallet_address.get() != msg::sender(){
            return Err("Only match making server can start a match".into());
        }
        Ok(())
    }

    //end a match
    pub fn end_match(&mut self, match_id: U256) -> Result<(), Vec<u8>>{
        if !self.initialized.get() {
            return Err("has not been initialized yet".into());
        }

        let current_match = self.matches.get(match_id);
        if !current_match.exists.get(){
            return Err("Match does not exist".into());
        }

        if self.matchmaking_server_wallet_address.get() != msg::sender(){
            return Err("Only match making server can start a match".into());
        }

        Ok(())
    }

}
