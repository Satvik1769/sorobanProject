#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, Address, Event, Result};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }
    pub fn xlm_sent(&self, from: Address, to: Address, amount: u128) -> Event {
        Event {
            topic: "xlmSent".to_string(),
            params: vec![
                ("_from".to_string(), from),
                ("_to".to_string(), to),
                ("_amount".to_string(), amount.to_string()),
            ],
        }
    }

    // Constructor to set the contract owner and accept Ether during deployment
    pub fn new(env: &Env) -> Self {
        Lock {
            owner: env.sender(),
        }
    }

    // Function to allow the owner to send Ether to a specified address
    pub fn send_xlm(&mut self, env: &Env, receiver: Address, amount: u128) -> Result<()> {
        env.require(env.sender() == self.owner, "Not the owner")?;
        env.require(receiver != Address::zero(), "Invalid receiver address")?;
        env.require(amount > 0, "Invalid amount")?;

        // Send Ether to the receiver
        env.send(receiver, amount)?;

        // Emit an event to log the successful transfer
        env.emit(&self.xlm_sent(env.sender(), receiver, amount));

        Ok(())
    }
}

mod test;
