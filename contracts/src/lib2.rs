//This is a new implementation of the project
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::{
    env, PanicOnDefault, Promise, AccountId, Balance
};
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct PoP {
    token_ids: u64,
    histories: UnorderedMap<u64, ChatHistory>,
}

#[derive(BorshDeserialize, BorshSerialize)]
struct ChatHistory {
    ipfs_hash: String,
    signer: AccountId,
    signature: Vec<u8>,
}

#[near_bindgen]
impl PoP {
    #[init]
    pub fn new() -> Self {
        Self {
            token_ids: 0,
            histories: UnorderedMap::new(b"histories".to_vec()),
        }
    }

    pub fn mint(&mut self, to: AccountId, ipfs_hash: String, signature: Vec<u8>) {
        let id = self.token_ids;
        let new_history = ChatHistory {
            ipfs_hash: ipfs_hash.clone(),
            signer: to.clone(),
            signature: signature.clone(),
        };
        self.histories.insert(id, history);
        self.token_ids +=1;

    }


    pub fn token_uri(&self, token_id: u64) -> Option<String> {
        self.histories.get(&token_id).map(|history| history.ipfs_hash)
    }


    pub fn token_signature(&self, token_id: u64) -> Option<Vec<u8>> {
        self.histories.get(&token_id).map(|history| history.signature)
    }



}








