//This is a new implementation of the project
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::{
    env, PanicOnDefault, Promise, AccountId, Balance
};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct PoP {
    token_ids: u64,
    
}

#[derive(BorshDeserialize, BorshSerialize)]
struct ChatHistory {
    ipfs_hash: String,
    signer: AccountId,
    signature: Vec<u8>,
}

#[]
