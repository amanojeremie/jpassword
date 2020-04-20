
use ring::{aead, digest, error};
use super::{hash::{pbkdf2_rand_salt, pbkdf2_with_salt, SALT_LEN}, rand::{generate_rand_vec}};

static AEAD_ALG: &'static aead::Algorithm = &aead::AES_256_GCM; 

const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

fn make_key<K: aead::BoundKey<OneNonceSequence>>(
    algorithm: &'static aead::Algorithm,
    key: &[u8],
    nonce: aead::Nonce,
) -> K {
    let key = aead::UnboundKey::new(algorithm, key).unwrap();
    let nonce_sequence = OneNonceSequence::new(nonce);
    K::new(key, nonce_sequence)
}

pub fn aead_seal(plaintext: &Vec<u8>, password: &[u8]) -> Result<Vec<u8>, ()> {
    
    let mut password_cred: Credential = [0u8; CREDENTIAL_LEN];
    let salt = pbkdf2_rand_salt(password, &mut password_cred);


    let nonce_vec = generate_rand_vec(AEAD_ALG.nonce_len()).unwrap();
    let nonce = aead::Nonce::try_assume_unique_for_key(&nonce_vec).unwrap();

    let mut key: aead::SealingKey<OneNonceSequence> = make_key(AEAD_ALG, &password_cred, nonce);
    let mut ciphertext = plaintext.clone();
    let seal_result = key.seal_in_place_append_tag(aead::Aad::empty(), &mut ciphertext);
    
    if seal_result.is_err() {    
        return Err(())    
    }

    ciphertext.extend(&salt);
    ciphertext.extend(&nonce_vec);
    Ok(ciphertext)
}

fn extract_ciphertext_salt_nonce(ciphertext_salt_nonce: &Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let ciphertext_slice = &ciphertext_salt_nonce
        [..ciphertext_salt_nonce.len() - AEAD_ALG.nonce_len() - SALT_LEN];
    let mut ciphertext = Vec::<u8>::new();
    ciphertext.extend_from_slice(ciphertext_slice);

    let salt_slice = &ciphertext_salt_nonce
        [ciphertext_salt_nonce.len() - AEAD_ALG.nonce_len() - SALT_LEN .. ciphertext_salt_nonce.len() - AEAD_ALG.nonce_len()];
    let mut salt = Vec::<u8>::new();
    salt.extend_from_slice(salt_slice);

    let nonce_slice = &ciphertext_salt_nonce
        [ciphertext_salt_nonce.len() - AEAD_ALG.nonce_len()..];
    let mut nonce = Vec::<u8>::new();
    nonce.extend_from_slice(nonce_slice);

    (ciphertext, salt, nonce)
}

pub fn aead_open(ciphertext_salt_nonce: &Vec<u8>, password: &[u8]) -> Result<Vec<u8>, ()> {
    let (ciphertext, salt, nonce) = extract_ciphertext_salt_nonce(&ciphertext_salt_nonce);

    let mut password_cred: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2_with_salt(password, &mut password_cred, &salt);

    let nonce = aead::Nonce::try_assume_unique_for_key(&nonce).unwrap();

    let mut plaintext = ciphertext.clone();
    let mut key: aead::OpeningKey<OneNonceSequence> = make_key(AEAD_ALG, &password_cred, nonce);
    let result = key.open_within(aead::Aad::empty(), &mut plaintext, std::ops::RangeFrom::<usize> { start: 0 });

    if result.is_err() {
        return Err(())
    }
    let plaintext_len = result.ok().unwrap().len();
    let plaintext_slice = &plaintext[..plaintext_len];
    let mut plaintext = Vec::<u8>::new();
    plaintext.extend_from_slice(plaintext_slice);
    Ok(plaintext)
}

struct OneNonceSequence(Option<aead::Nonce>);

impl OneNonceSequence {
    /// Constructs the sequence allowing `advance()` to be called
    /// `allowed_invocations` times.
    fn new(nonce: aead::Nonce) -> Self {
        Self(Some(nonce))
    }
}

impl aead::NonceSequence for OneNonceSequence {
    fn advance(&mut self) -> Result<aead::Nonce, error::Unspecified> {
        self.0.take().ok_or(error::Unspecified)
    }
}