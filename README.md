# tweetnacl-rs

re-export from [sodalite][1].

## Usage

```rust
use tweetnacl_rs::gen;
use tweetnacl_rs::TweetNacl;

fn main() {
    let (pk, sk) = gen();
    println!("public key: {:?}", pk);
    println!("secret key: {:?}", sk.to_vec());

    let sm = sk.sign("hello, world");    
    println!("signed message: {:?}", sm.to_vec());

    let ret = pk.verify("hello, world", &sm);
    assert_eq!(ret, true);
}
```

## License

[MIT][2]

[1]: https://crates.io/crates/sodalite
[2]: https://choosealicense.com/licenses/mit/
