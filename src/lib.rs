use rand::RngCore;

type Seed = [u8; 32];
type PublicKey = [u8; 32];
type SecretKey = [u8; 64];

/// Generate a keypair from a random seed.
pub fn gen() -> (PublicKey, SecretKey) {
    let mut pk: PublicKey = [0; 32];
    let mut sk: SecretKey = [0; 64];
    let mut seed: Seed = [0; 32];
    ::rand::thread_rng().fill_bytes(&mut seed);

    ::sodalite::sign_keypair_seed(&mut pk, &mut sk, &seed);
    
    (pk, sk)
}

pub fn to_hex<T: std::convert::AsRef<[u8]>>(bytes: T) -> String {
    hex::encode(bytes)
}

pub fn from_hex<T: std::convert::AsRef<[u8]>>(string: T) -> Vec<u8> {
    hex::decode(string).unwrap()
}

/// `TweetNacl` Trait for slice.
///
/// # Usage
/// ```rust
/// use tweetnacl_rs::TweetNacl;
/// 
/// fn main() {
///     let (pk, sk) = [0; 32].gen();
///     println!("public key: {:?}", pk);
///     println!("secret key: {:?}", sk.to_vec());
/// 
///     let sm = sk.sign("hello, world".to_string());
///     println!("signed message: {:?}", sm.to_vec());
/// 
///     let ret = pk.verify("hello, world".to_string(), &sm);
///     assert_eq!(ret, true);
/// }
/// ```
pub trait TweetNacl {
    fn gen(self) -> (PublicKey, SecretKey);
    fn sign(self, _m: String) -> Vec<u8>;
    fn verify(self, _m: String, _sm: &Vec<u8>) -> bool;
}

impl TweetNacl for &[u8] {
    /// Generate a keypair from seed.
    ///
    /// # Panics
    /// Seed should be `[u8; 32]`.
    fn gen(self) -> (PublicKey, SecretKey) {
        assert_eq!(self.len(), 32);
        
        let mut pk: PublicKey = [0; 32];
        let mut sk: SecretKey = [0; 64];
        let mut seed: Seed = [0; 32];
        
        seed.copy_from_slice(self);
        ::sodalite::sign_keypair_seed(&mut pk, &mut sk, &seed);
        
        (pk, sk)
    }

    /// Sign a message using secret key.
    fn sign(self, _m: String) -> Vec<u8> {
        let mut m: Vec<u8> = vec![];
        let mut sm: Vec<u8> = vec![0; 64];
        let mut sk: SecretKey = [0; 64];

        sk.copy_from_slice(self);
        m.append(_m.as_bytes().to_vec().as_mut());
        sm.append(_m.as_bytes().to_vec().as_mut());
        ::sodalite::sign_attached(sm.as_mut(), &m, &sk);

        sm
    }
    
    /// Verify signature.
    ///
    /// # Panics
    /// + PublicKey should be `[u8; 32]`.
    /// + __Signed Message__'s length `minus` __Message__'s length shoule be 64.
    fn verify(self, _m: String, _sm: &Vec<u8>) -> bool {
        let mut m: Vec<u8> = vec![0; 64];
        let mut sm: Vec<u8> = vec![];

        m.append(_m.as_bytes().to_vec().as_mut());
        sm.append(_sm.to_vec().as_mut());
        
        assert_eq!(self.len(), 32);
        assert_eq!(sm.len() - m.len(), 0);

        let mut pk: PublicKey = [0; 32];
        pk.copy_from_slice(self);
        
        ::sodalite::sign_attached_open(&mut m, &sm, &pk).is_ok()
    }
}
