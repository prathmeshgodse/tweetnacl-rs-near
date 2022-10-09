use tweetnacl_rs_near::TweetNacl;
use tweetnacl_rs_near::{from_hex, to_hex};

fn help() {
    println!(
        r#"tweetnacl-rs
============
Usage: tweetnacl-rs <Command> <args>

Commands:
  gen     <nil>                               => generate keypair
  sign    <message> <secretkey>               => sign message
  verify  <message> <signature> <publickey>   => verify message
"#
    );
}

/// test-public-key:
/// ab95e0cadd3899e0b34e1681716bda442cf431ed9cb76fc1d0c3ea82dc461782
///
/// test-secret-key:
/// 87a7ea8f0ea07709f46104682c123b87875e406f2d07e3cc3f0a41b62287a037ab95e0cadd3899e0b34e1681716bda442cf431ed9cb76fc1d0c3ea82dc461782
///
/// sign-public-key:
/// 16e4fca50236fb9e0a106e445ff7630d54eb650da6cc4907556cefb14d16fca681d54ddc2c47e01ab3e5ee51c28a84a1ed7311df7c5e8ccfa53152a79be3f40a61623935653063616464333839396530623334653136383137313662646134343263663433316564396362373666633164306333656138326463343631373832
fn main() {
    let mut args = vec![];
    for arg in std::env::args() {
        args.push(arg);
    }

    if args.len() == 1 {
        help();
        return;
    }

    let cmd = args[1].as_str();
    match cmd {
        "gen" => {
            let (pk, sk) = tweetnacl_rs_near::gen();
            println!("public-key: {:?}", tweetnacl_rs_near::to_hex(pk));
            println!("secret-key: {:?}", tweetnacl_rs_near::to_hex(sk.to_vec()));
        }
        "sign" => {
            if args[3].to_owned().len() != 128 {
                println!("<secret-key's length should be 128>");
                return;
            }

            let sk = from_hex(String::from(args[3].to_owned()));
            let sign = sk.sign(String::from(args[2].to_owned()));

            println!("message: {:?}\n", args[2].to_owned());
            println!("secret-key: {:?}\n", args[3].to_owned());
            println!("signatrue-hex: {:?}", to_hex(sign));
        }
        "verify" => {
            if args[4].to_owned().len() != 64 {
                println!("<public-key's length should be 64>");
                return;
            }

            let pk = from_hex(String::from(args[4].to_owned()));
            let ret = pk.verify(
                String::from(args[2].to_owned()),
                &from_hex(String::from(args[3].to_owned())),
            );

            println!("message: {:?}\n", args[2].to_owned());
            println!("signatrue-hex: {:?}\n", args[3].to_owned());
            println!("verification: {:?}", ret);
        }
        _ => help(),
    }
}
