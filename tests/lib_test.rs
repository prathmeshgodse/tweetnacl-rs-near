use tweetnacl_rs::gen;
use tweetnacl_rs::TweetNacl;

#[test]
fn test_gen() {
    let (pk, sk) = gen();
    assert_eq!(pk.len(), 32);
    assert_eq!(sk.len(), 64);

    let sm = sk.sign("hello, world");
    let ret = pk.verify("hello, world", &sm);

    assert_eq!(sm.len(), 76);
    assert_eq!(ret, true);
}

#[test]
#[should_panic]
fn test_panic_verify() {
    let (pk, sk) = gen();
    assert_eq!(pk.len(), 32);
    assert_eq!(sk.len(), 64);

    let sm = sk.sign("hello, world");
    assert_eq!(sm.len(), 76);

    pk.verify("foo", &sm);
}

#[test]
fn test_seed_tweetnacl() {
    let (pk, sk) = [0; 32].gen();
    
    assert_eq!(pk, [
        59, 106, 39, 188, 206, 182, 164, 45,
        98, 163, 168, 208, 42, 111, 13, 115,
        101, 50, 21, 119, 29, 226, 67, 166,
        58, 192, 72, 161, 139, 89, 218, 41
    ]);
    assert_eq!(sk.to_vec(), vec![
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        59, 106, 39, 188, 206, 182, 164, 45,
        98, 163, 168, 208, 42, 111, 13, 115,
        101, 50, 21, 119, 29, 226, 67, 166,
        58, 192, 72, 161, 139, 89, 218, 41
    ]);

    let sm = sk.sign("hello, world");
    assert_eq!(sm.to_vec(), vec![
        178, 139, 95, 24, 176, 228, 2, 204,
        104, 121, 144, 26, 157, 219, 132, 195,
        205, 29, 21, 7, 74, 208, 59, 140,
        97, 126, 63, 253, 119, 248, 66, 155,
        113, 111, 25, 203, 169, 55, 91, 39,
        54, 100, 189, 49, 191, 137, 42, 56,
        108, 3, 27, 177, 249, 2, 76, 113,
        128, 22, 84, 227, 138, 23, 169, 9,
        104, 101, 108, 108, 111, 44, 32, 119,
        111, 114, 108, 100
    ]);

    let ret = pk.verify("hello, world", &sm);
    assert_eq!(ret, true);    
}
