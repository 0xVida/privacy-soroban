use soroban_sdk::{Env, Vec, vec, U256, crypto::BnScalar};
use soroban_poseidon::poseidon_hash;

pub struct MerkleTree {
    pub height: u32,
    pub zero_hashes: Vec<U256>,
}

impl MerkleTree {
    pub fn new(env: &Env, height: u32) -> Self {
        let mut zero_hashes = vec![env];
        
        let mut current_hash = U256::from_u32(env, 0);
        zero_hashes.push_back(current_hash.clone());

        //  zero hashes for each level
        for _ in 0..height {
            let inputs = vec![env, current_hash.clone(), current_hash.clone()];
            current_hash = poseidon_hash::<3, BnScalar>(env, &inputs);
            zero_hashes.push_back(current_hash.clone());
        }

        Self { height, zero_hashes }
    }
 
    ///  if leaves < 2^height it pads with precomputed zero hashes
    pub fn compute_root(&self, env: &Env, leaves: Vec<U256>) -> U256 {
        let mut current_level = leaves;
        let max_leaves = 2u32.pow(self.height);
        
        // Pad leafs to reach the full width of the tree at level 0
        while current_level.len() < max_leaves {
            current_level.push_back(self.zero_hashes.get(0).unwrap());
        }

        for _level in 0..self.height {
            let mut next_level = vec![env];
            for i in (0..current_level.len()).step_by(2) {
                let left = current_level.get(i).unwrap();
                let right = current_level.get(i + 1).unwrap();
                
                let inputs = vec![env, left, right];
                let parent = poseidon_hash::<3, BnScalar>(env, &inputs);
                next_level.push_back(parent);
            }
            current_level = next_level;
        }

        current_level.get(0).unwrap()
    }

    /// Generates a proof for a leaf at a given index.returns a list of sibling hashes
    pub fn get_proof(&self, env: &Env, leaves: Vec<U256>, index: u32) -> Vec<U256> {
        let mut proof = vec![env];
        let mut current_level = leaves;
        let mut current_index = index;

        let max_leaves = 2u32.pow(self.height);
        while current_level.len() < max_leaves {
            current_level.push_back(self.zero_hashes.get(0).unwrap());
        }

        for _level in 0..self.height {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            proof.push_back(current_level.get(sibling_index).unwrap());

            let mut next_level = vec![env];
            for i in (0..current_level.len()).step_by(2) {
                let left = current_level.get(i).unwrap();
                let right = current_level.get(i + 1).unwrap();
                let inputs = vec![env, left, right];
                next_level.push_back(poseidon_hash::<3, BnScalar>(env, &inputs));
            }
            current_level = next_level;
            current_index /= 2;
        }

        proof
    }

    pub fn verify_proof(env: &Env, root: U256, leaf: U256, proof: Vec<U256>, index: u32) -> bool {
        let mut current = leaf;
        let mut current_index = index;

        for sibling in proof.iter() {
            let inputs = if current_index % 2 == 0 {
                vec![env, current, sibling]
            } else {
                vec![env, sibling, current]
            };
            current = poseidon_hash::<3, BnScalar>(env, &inputs);
            current_index /= 2;
        }

        current == root
    }
}
