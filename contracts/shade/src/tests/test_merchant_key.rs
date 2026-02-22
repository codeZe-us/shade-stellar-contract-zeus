#![cfg(test)]

use crate::shade::{Shade, ShadeClient};
use soroban_sdk::testutils::{Address as _, Events as _};
use soroban_sdk::{Address, Env, String};

#[test]
fn test_set_merchant_key_success() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);

    let key = String::from_str(&env, "GC3L4...EXAMPLE");
    client.set_merchant_key(&merchant, &key);

    assert_eq!(client.get_merchant_key(&merchant), key);

    let events = env.events().all();
    assert!(events.len() > 0, "No events captured after registration and key set!");
}

