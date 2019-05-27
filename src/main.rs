fn main() {
    let (pk, sk) = tweetnacl_rs::gen();

    println!("public-key: {:?}", tweetnacl_rs::to_hex(pk));
    println!("secret-key: {:?}", tweetnacl_rs::to_hex(sk.to_vec()));
}
