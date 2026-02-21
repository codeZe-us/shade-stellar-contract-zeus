#![cfg(test)]

use crate::shade::{Shade, ShadeClient};
use crate::errors::ContractError;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env, String};

fn setup_test() -> (Env, ShadeClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);
    (env, client, contract_id, admin)
}

#[test]
fn test_set_merchant_key_success() {
    let (env, client, _contract_id, _admin) = setup_test();

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);

    let key = String::from_str(&env, "GC3L4...EXAMPLE");
    client.set_merchant_key(&merchant, &key);

    assert_eq!(client.get_merchant_key(&merchant), key);
}


#[test]
fn test_update_merchant_key() {
    let (env, client, _contract_id, _admin) = setup_test();

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);

    let key1 = String::from_str(&env, "KEY_ONE");
    client.set_merchant_key(&merchant, &key1);
    assert_eq!(client.get_merchant_key(&merchant), key1);

    let key2 = String::from_str(&env, "KEY_TWO");
    client.set_merchant_key(&merchant, &key2);
    assert_eq!(client.get_merchant_key(&merchant), key2);
}

#[test]
fn test_set_merchant_key_authorization() {
    let (env, client, _contract_id, _admin) = setup_test();

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);

    let _hacker = Address::generate(&env);
    let malicious_key = String::from_str(&env, "HACKED");

    let unregistered = Address::generate(&env);
    let res = client.try_set_merchant_key(&unregistered, &malicious_key);
    assert!(res.is_err()); 
    
    let expected_error = soroban_sdk::Error::from_contract_error(ContractError::MerchantNotFound as u32);
    assert_eq!(res.unwrap_err().unwrap(), expected_error);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #11)")]
fn test_get_non_existent_key() {
    let (env, client, _contract_id, _admin) = setup_test();

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);

    client.get_merchant_key(&merchant);
}

