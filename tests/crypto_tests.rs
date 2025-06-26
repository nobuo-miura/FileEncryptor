use fileencryptor::crypto::{decrypt_data, encrypt_data};

#[test]
fn encrypt_decrypt_round_trip() {
    let plaintext = b"Hello, world!";
    let password = "correct horse battery staple";

    let encrypted = encrypt_data(plaintext, password);
    let decrypted = decrypt_data(&encrypted, password).expect("decryption failed");

    assert_eq!(decrypted, plaintext);
}

#[test]
fn decrypt_with_wrong_password_fails() {
    let plaintext = b"Secret data";
    let password = "letmein";
    let wrong_password = "incorrect";

    let encrypted = encrypt_data(plaintext, password);
    assert!(decrypt_data(&encrypted, wrong_password).is_err());
}
