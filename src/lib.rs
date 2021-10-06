use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

// Near Collections

use near_sdk::collections::{TreeMap};


near_sdk::setup_alloc!();

// Typedef in Rust
pub type ScoreType = u32;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ScoreBoard{
    pub table: TreeMap<String, ScoreType>,
}

impl Default for ScoreBoard {
    fn default() -> Self {
        env::panic(b"The contract is not initialized.")
    }
}

#[near_bindgen]
impl ScoreBoard {
    /// Init attribute used for instantiation.
    #[init]
    pub fn new() -> Self {
        // Useful snippet to copy/paste, making sure state isn't already initialized
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        // Note this is an implicit "return" here
        Self {
            table: TreeMap::new(b"v".to_vec()),
        }
    }

    pub fn update_score(&mut self, score: ScoreType, name: String) {
        let existing_score: Option<ScoreType> = self.table.get(&name.clone());
        if existing_score.is_some() {
            env::log(b"Checking if score has increased");
            if self.table.get(&name.clone()) < Some(score){
                self.table.insert(&name.clone(), &score);
                env::log(b"Score Updated")
            }
            else{
                env::log(b"Score Same")
            }
        }
        self.table.insert(&name.into(), &score);
    }

    pub fn lowest_scorer(&mut self) -> String {
        match self.table.min(){
            Some(lowest_scorer_id) =>{
                lowest_scorer_id
        },
        None => "Not initialized".to_string()
        }
    }

    pub fn highest_scorer(&mut self) -> String {
        match self.table.min(){
            Some(highest_scorer_id) =>{
                highest_scorer_id
        },
        None => "Not initialized".to_string()
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
