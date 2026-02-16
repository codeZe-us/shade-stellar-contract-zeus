use soroban_sdk::{contractevent, Address, Env};

#[contractevent]
pub struct InitalizedEvent {
    pub admin: Address,
    pub timestamp: u64,
}

pub fn publish_initialized_event(env: &Env, admin: Address, timestamp: u64) {
    InitalizedEvent { admin, timestamp }.publish(env);
}
