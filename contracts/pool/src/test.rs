#![cfg(test)]
extern crate std;
use super::*;
use soroban_sdk::{Env, BytesN, vec, U256};

#[test]
fn test_merkle_tree_logic() {
    let env = Env::default();
    let contract_id = env.register(PrivacyPool, ());
    let client = PrivacyPoolClient::new(&env, &contract_id);

    let height = 3;
    let mt = merkle::MerkleTree::new(&env, height);
 
    let mut leaves = vec![&env];
    for i in 0..8 {
        let secret = BytesN::from_array(&env, &[i as u8; 32]);
        leaves.push_back(client.compute_commitment(&(1000 + i as u128), &secret));
    }

    // compute root
    let root = mt.compute_root(&env, leaves.clone());
    std::println!("Merkle Root: {:?}", root);

    //generate and verify proof for leaf at index 2
    let index = 2;
    let leaf = leaves.get(index).unwrap();
    let proof = mt.get_proof(&env, leaves.clone(), index);
    
    assert!(merkle::MerkleTree::verify_proof(&env, root.clone(), leaf.clone(), proof.clone(), index));
    std::println!("Proof for index {} verified!", index);

    // verify invalid proof fail
    let invalid_leaf = U256::from_u32(&env, 999);
    assert!(!merkle::MerkleTree::verify_proof(&env, root, invalid_leaf, proof, index));
}
