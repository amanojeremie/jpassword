use ring::{rand::{self, SecureRandom}};

/// Generate a random vector of bytes of specified length
/// # Arguments
///
/// * `len` - The length of the vector to randomly generate
pub fn generate_rand_vec(len: usize) -> Result<Vec<u8>, ()> {
    let mut salt = vec!(0u8; len);
    let rng = rand::SystemRandom::new();
    let result = rng.fill(&mut salt);
    if result.is_err() {
        return Err(())
    }
    Ok(salt)
}