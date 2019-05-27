# tweetnacl-rs
[![Crates.io](https://img.shields.io/crates/v/tweetnacl-rs.svg)](https://crates.io/crates/tweetnacl-rs)
[![Crates.io](https://img.shields.io/crates/d/tweetnacl-rs.svg)](https://crates.io/crates/tweetnacl-rs)
[![LICENSE](https://img.shields.io/crates/l/tweetnacl-rs.svg)](https://crates.io/crates/tweetnacl-rs)

re-export from [sodalite][1].

## Usage

You can test the keypair in command line directly, for example:

```shell
$ cargo install tweetnacl-rs
$ tweetnacl-rs
tweetnacl-rs
============
Usage: tweetnacl-rs <Command> <args>

Commands:
  gen     <nil>                               => generate keypair
  sign    <message> <secretkey>               => sign message
  verify  <message> <signature> <publickey>   => verify message

$ tweetnacl-rs gen
public-key: "aa73d5a6c526bbad121a0c44d84cf31ae705bbadaf1221759a4da10e6d8fdcf1"
secret-key: "cb6e6713a8c2aba686b0ad4d7d914f1a9a31ee450a3d60b7d979131c045066a9aa73d5a6c526bbad121a0c44d84cf31ae705bbadaf1221759a4da10e6d8fdcf1"
```

Or your can use `tweetnacl-rs` as a library:

```rust
use tweetnacl_rs::gen;
use tweetnacl_rs::TweetNacl;

fn main() {
    let (pk, sk) = gen();
    println!("public key: {:?}", pk);
    println!("secret key: {:?}", sk.to_vec());

    let sm = sk.sign("hello, world".to_string());
    println!("signed message: {:?}", sm.to_vec());

    let ret = pk.verify("hello, world".to_string(), &sm);
    assert_eq!(ret, true);
}
```

## LICENSE

[MIT][2]

[1]: https://crates.io/crates/sodalite
[2]: https://choosealicense.com/licenses/mit/
