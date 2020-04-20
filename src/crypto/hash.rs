use ring::{pbkdf2, digest::{self, digest}};
use super::{rand::generate_rand_vec};
use std::{num::NonZeroU32};

const PBKDF2_ITER: u32 = 20_000;
pub const SALT_LEN: usize = 32;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
static HASH_ALG: &digest::Algorithm = &digest::SHA256;

pub fn pbkdf2_rand_salt(password: &[u8], in_out: &mut[u8]) -> Vec<u8> {
    let salt = generate_rand_vec(SALT_LEN).unwrap();

    pbkdf2_with_salt(password, in_out, &salt);

    salt
}

pub fn pbkdf2_with_salt(password: &[u8], in_out: &mut[u8], salt: &[u8]) {
    pbkdf2::derive(PBKDF2_ALG, NonZeroU32::new(PBKDF2_ITER).unwrap(), &salt,
    password, in_out);
}

pub fn pbkdf2_verify(password: &[u8], _in: &[u8], salt: &[u8]) -> Result<(), ()> {
    let result = pbkdf2::verify(PBKDF2_ALG, NonZeroU32::new(PBKDF2_ITER).unwrap(), salt, password, _in);
    if result.is_err() {
        return Err(())
    }
    Ok(())
}

pub fn hash(to_hash: &[u8]) -> Vec<u8> {
    digest(HASH_ALG, to_hash).as_ref().to_vec()
}