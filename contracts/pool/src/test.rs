#![cfg(test)]
extern crate std;
use super::*;
use soroban_sdk::{Env, BytesN, vec};

#[test]
fn test_mk_root_height_3() {
    let env = Env::default();
    let contract_id = env.register(PrivacyPool, ());
    let client = PrivacyPoolClient::new(&env, &contract_id);

    // we create 8 leaves (Height 3)
    let mut nodes = vec![&env];
    for i in 0..8 {
        let secret = BytesN::from_array(&env, &[i as u8; 32]);
        nodes.push_back(client.compute_commitment(&(1000 + i as u128), &secret));
    }

    // were iteratively hashing until we reach the root
    let mut current_level = nodes;
    while current_level.len() > 1 {
        let mut next_level = vec![&env];
        for i in (0..current_level.len()).step_by(2) {
            let left = current_level.get(i).unwrap();
            let right = current_level.get(i + 1).unwrap();
            
            let inputs = vec![&env, left, right];
            let parent = poseidon_hash::<3, BnScalar>(&env, &inputs);
            next_level.push_back(parent);
        }
        current_level = next_level;
    }

    let root = current_level.get(0).unwrap();
    std::println!("our final root (height 3): {:?}", root);
}

