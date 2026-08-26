#![no_std]
use soroban_sdk::{contract, contractimpl, BytesN, Env, Vec};

#[contract]
pub struct CryptoInLoopFixedContract;

#[contractimpl]
impl CryptoInLoopFixedContract {
    pub fn verify_fixed(env: Env, items: Vec<(BytesN<32>, BytesN<32>, BytesN<64>)>) {
        for (pk, msg, sig) in items.iter() {
            env.crypto().ed25519_verify(&pk, &msg, &sig);
        }
    }
}
