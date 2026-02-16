use soroban_sdk::{contracttype, Address};

#[contracttype]
pub enum DataKey {
    Admin,
    ContractInfo,
}

#[contracttype]
pub struct ContractInfo {
    pub admin: Address,
    pub timestamp: u64,
}
