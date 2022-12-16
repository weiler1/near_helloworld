// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};
use rand::Rng;

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    message: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{message: DEFAULT_MESSAGE.to_string()}
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        log!("Saving greeting {}", message);
        self.message = message;
    }

    // Public method - sets current greeting to its negative
    pub fn angry_greeting(&mut self) {
        log!("Greeting Angrily Now");
        let new_message = format!("{}{}", "Not ".to_string(), self.message.clone());
        self.message = new_message;
    }

    pub fn random_greeting(&mut self) {
        log!("Greeting Randomly Now");
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..4);
        self.message = match x{
            0 => "Hi".to_string(),
            1 => "Hola".to_string(),
            2 => "Konnichiwa".to_string(),
            3 => "Salam".to_string(),
            4 => "Namaste".to_string(),
            _ => "Aloha".to_string(),
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn check_wrong_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_ne!(
            contract.get_greeting(),
            "Konnichiwa".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }

    #[test]
    fn get_angry_greeting() {
        let mut contract = Contract::default();
        contract.angry_greeting();
        assert_eq!(
            contract.get_greeting(),
            "Not Hello".to_string()
        );
    }
}
