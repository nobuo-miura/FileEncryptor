use aes_gcm::aead::{Aead, OsRng, rand_core::RngCore};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};

pub fn derive_key_from_password(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    let argon2 = Argon2::default();

    let salt_str = SaltString::encode_b64(salt).unwrap();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt_str)
        .unwrap();
    let derived = hash.hash.unwrap();

    key.copy_from_slice(derived.as_bytes());
    key
}

pub fn encrypt_data(plaintext: &[u8], password: &str) -> Vec<u8> {
    let mut salt = [0u8; 16];
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce_bytes);

    let key = derive_key_from_password(password, &salt);
    let cipher = Aes256Gcm::new(&key.into());
    let nonce = Nonce::from(nonce_bytes);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .expect("encryption failed");

    // [salt][nonce][ciphertext]
    let mut output = vec![];
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);
    output
}

pub fn decrypt_data(encrypted: &[u8], password: &str) -> Result<Vec<u8>, &'static str> {
    if encrypted.len() < 16 + 12 {
        return Err("Input too short");
    }

    let salt = &encrypted[0..16];
    let nonce_bytes = &encrypted[16..28];
    let ciphertext = &encrypted[28..];

    let key = derive_key_from_password(password, salt);
    let cipher = Aes256Gcm::new(&key.into());
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "decryption failed")
}
