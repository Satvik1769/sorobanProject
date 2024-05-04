#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, vec, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol_short!("Dev"));
    assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    );
    let balance =  client.xlm_sent(&symbol_short("GCMZUY3JMHNQ4KMPNX5ZJOGVUBN6TQ3LROMTR5V3JSNWXO2MDLI6RRK5"),&symbol_short("GBD336B7XVHZZGVY5ETEB2OVGZRQ3XGBFXTBQ6PPNPWJUMLEYQJWYMK2"),32)
}
