#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, BytesN, Env, Vec, vec, U256, crypto::BnScalar};
use soroban_poseidon::poseidon_hash;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Commitments,
}

#[contract]
pub struct PrivacyPool;

#[contractimpl]
impl PrivacyPool {
    pub fn deposit(env: Env, commitment: BytesN<32>) {
        
        let mut commitments: Vec<BytesN<32>> = env
            .storage()
            .persistent()
            .get(&DataKey::Commitments)
            .unwrap_or(vec![&env]);

        commitments.push_back(commitment);

        env.storage().persistent().set(&DataKey::Commitments, &commitments);
    }

    pub fn get_commitments(env: Env) -> Vec<BytesN<32>> {
        env.storage()
            .persistent()
            .get(&DataKey::Commitments)
            .unwrap_or(vec![&env])
    }

    pub fn compute_commitment(env: Env, amount: u128, secret: BytesN<32>) -> U256 {
        let amount_u256 = U256::from_u128(&env, amount);
        let secret_u256 = U256::from_be_bytes(&env, &secret.into());
        
        let inputs = vec![&env, amount_u256, secret_u256];
        
        poseidon_hash::<3, BnScalar>(&env, &inputs)
    }
}

pub mod merkle;
mod test;
