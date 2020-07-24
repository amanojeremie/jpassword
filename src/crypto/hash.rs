use ring::{pbkdf2, digest::{self, digest}};
use super::{rand::generate_rand_vec};
use std::{num::NonZeroU32};

const PBKDF2_ITER: u32 = 20_000;
pub const SALT_LEN: usize = 32;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
static HASH_ALG: &digest::Algorithm = &digest::SHA256;

/// Creates a password based key using PBKDF2, returning the random salt used in its creation
/// # Arguments
///
/// * `password` - The array of bytes used as a password
/// * `in_out` - The mutable array of bytes to write the key to
pub fn pbkdf2_rand_salt(password: &[u8], in_out: &mut[u8]) -> Vec<u8> {
    let salt = generate_rand_vec(SALT_LEN).unwrap();

    pbkdf2_with_salt(password, in_out, &salt);

    salt
}

/// Creates a password based key using a given salt
/// # Arguments
///
/// * `password` - The array of bytes used as a password
/// * `in_out` - The mutable array of bytes to write the key to
/// * `salt` - The salt used for the PBKDF2 function
pub fn pbkdf2_with_salt(password: &[u8], in_out: &mut[u8], salt: &[u8]) {
    pbkdf2::derive(PBKDF2_ALG, NonZeroU32::new(PBKDF2_ITER).unwrap(), &salt,
    password, in_out);
}

/// Verifies a password based key using a password and salt
/// # Arguments
///
/// * `password` - The array of bytes used as a password
/// * `in` - The key to test the password on salt on
/// * `salt` - The salt used for the PBKDF2 function
pub fn pbkdf2_verify(password: &[u8], _in: &[u8], salt: &[u8]) -> Result<(), ()> {
    let result = pbkdf2::verify(PBKDF2_ALG, NonZeroU32::new(PBKDF2_ITER).unwrap(), salt, password, _in);
    if result.is_err() {
        return Err(())
    }
    Ok(())
}

/// Hashes an array of bytes, returning a vector of bytes as the hash
/// # Arguments
///
/// * `to_hash` - The array of bytes to hash
pub fn hash(to_hash: &[u8]) -> Vec<u8> {
    digest(HASH_ALG, to_hash).as_ref().to_vec()
}