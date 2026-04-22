use poseidon;

fn main(secret: Field, commitment: pub Field) {
    let hash = poseidon::poseidon::bn254::hash_1([secret]);
    assert(hash == commitment);
}

#[test]
fn test_poseidon() {
    let secret = 123;
    let hash = poseidon::poseidon::bn254::hash_1([secret]);
    std::println::<Field>(hash);
}
