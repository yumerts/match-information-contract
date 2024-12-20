//!
//! Stylus Match Information Contract
//! 
//! This contract is used to store match related information

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::{Address, U8};
use alloy_sol_types::sol;
// use ethers::{core::k256::ecdsa::hazmat::verify_prehashed, etherscan::verify};
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, evm, msg, prelude::*, storage::{StorageAddress, StorageBool, StorageMap, StorageU256, StorageU8}};

sol_interface! {
    interface IPlayerInfoContract {   
        function addMatchResults(address winner_address, address loser_address) external;
    }

    interface IPredictionContract{
        // use this after two players have join the same match
        function createPredictionPool(uint256 match_id) external;

        // use this to block prediction after the match starts
        function stopAllowPrediction(uint256 match_id) external;

        // use this to submit prediction
        function submitMatchResults(uint256 match_id, uint256 winner) external;
    }
}

sol!{
    event matchCreated(uint256 indexed match_id, address indexed player1);
    event matchJoined(uint256 indexed match_id, address indexed player2);
    event matchStarted(uint256 indexed match_id, address indexed player1, address indexed player2);
    event matchEnded(uint256 indexed match_id, address indexed winner);
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
    state: StorageU8 //0 for Finding, 1 for Matched, 2 for Ready (for Prediction), 3 for Started, 4 for Ended
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
        self.initialized.set(true);
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

    //create a match
    //set the sender as player 1
    pub fn create_match(&mut self) -> Result<(), Vec<u8>>{
       
        if !self.initialized.get() {
            return Err("has not been initialized yet".into());
        }

        let latest_match_id = self.latest_match_id.get();

        //add match to the storage
        let mut match_setter = self.matches.setter(latest_match_id);
        match_setter.exists.set(true);
        match_setter.player1.set(msg::sender());
        match_setter.state.set(U8::from(0));

        let match_id = self.latest_match_id.get();
        self.latest_match_id.set(match_id + U256::from(1));
        
        evm::log(
            matchCreated{
                match_id: match_id,
                player1: msg::sender()
            }
        );
        Ok(())
    }

    //join a match
    pub fn join_match(&mut self, match_id: U256) -> Result<(), Vec<u8>>{
        
        if !self.initialized.get() {
            return Err("has not been initialized yet".into());
        }

        //check if the match exists
        let current_match = self.matches.get(match_id);
        if !current_match.exists.get(){
            return Err("Match does not exist".into());
        }

        //check if the match is in the finding state
        if current_match.state.get() != U8::from(0){
            return Err("Match is not in the finding state".into());
        }

        //set the sender as player 2
        let mut match_setter = self.matches.setter(match_id);
        match_setter.player2.set(msg::sender());
        match_setter.state.set(U8::from(1));

        evm::log(
            matchJoined{
                match_id: match_id,
                player2: msg::sender()
            }
        );

        Ok(())
    }

    pub fn open_prediction_market(&mut self, match_id: U256) -> Result<(), Vec<u8>>{
        if !self.initialized.get() {
            return Err("has not been initialized yet".into());
        }

        let current_match = self.matches.get(match_id);
        if !current_match.exists.get(){
            return Err("Match does not exist".into());
        }

        if current_match.state.get() != U8::from(1){
            return Err("Match is not in the matched state".into());
        }

        if self.matchmaking_server_wallet_address.get() != msg::sender(){
            return Err("Only match making server can open prediction market".into());
        }

        let mut match_setter = self.matches.setter(match_id);
        match_setter.state.set(U8::from(2));

        let prediction_contract = IPredictionContract::new(self.prediction_smart_contract_address.get());
        let prediction_pool_creation = prediction_contract.create_prediction_pool(self, match_id);
        if prediction_pool_creation.is_err(){
            return Err("Error Creating Prediction Pool".into());
        }

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

        if current_match.state.get() != U8::from(2){
            return Err("Match is not in the ready state".into());
        }

        if self.matchmaking_server_wallet_address.get() != msg::sender(){
            return Err("Only match making server can start a match".into());
        }

        let player1 = current_match.player1.get();
        let player2 = current_match.player2.get();

        //change match state to started
        let mut match_setter = self.matches.setter(match_id);
        match_setter.state.set(U8::from(3));

        let prediction_contract = IPredictionContract::new(self.prediction_smart_contract_address.get());
        let prediction_pool_creation = prediction_contract.stop_allow_prediction(self, match_id);
        if prediction_pool_creation.is_err(){
            return Err("Error Stopping Prediction".into());
        }

        evm::log(
            matchStarted{
                match_id: match_id,
                player1: player1,
                player2: player2
            }
        );

        Ok(())
    }

    //end a match
    pub fn end_match(&mut self, match_id: U256, winner: U256) -> Result<(), Vec<u8>>{
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

        let player1 = current_match.player1.get();
        let player2 = current_match.player2.get();

        //change match state to ended
        let mut match_setter = self.matches.setter(match_id);
        match_setter.state.set(U8::from(4));
        
        let update_player_info_result = self.update_player_info(winner, player1, player2);
        if update_player_info_result.is_err(){
            return Err(update_player_info_result.unwrap_err().into());
        }

        let prediction_contract = IPredictionContract::new(self.prediction_smart_contract_address.get());
        let prediction_pool_creation = prediction_contract.submit_match_results(self, match_id, winner);
        if prediction_pool_creation.is_err(){
            return Err("Error Submitting Prediction".into());
        }

        evm::log(
            matchEnded{
                match_id: match_id,
                winner: if winner == U256::from(1) { player1 } else { player2 }
            }
        );

        Ok(())
    }

}

impl MatchInformationContract{
    fn update_player_info(&mut self, winner: U256, player1: Address, player2: Address) -> Result<(), Vec<u8>>{
        let player_info_contract = IPlayerInfoContract::new(self.player_info_smart_contract_address.get());
        let player_info_update =  player_info_contract.add_match_results(self, if winner == U256::from(1) { player1 } else { player2 }, if winner == U256::from(1) { player2 } else { player1 });
        if player_info_update.is_err() {
            return Err(player_info_update.unwrap_err().into());
        }
        Ok(())
    }
}