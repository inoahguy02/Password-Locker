use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use std::error::Error;
use argon2::Argon2;


pub fn encrypt(data: String) {

}

pub fn decrypt(data: String) {

}

pub fn hash(pass: String) -> Vec<u8> {
    // gen salt
    let pwd = pass.as_bytes();
    let salt_str = SaltString::generate(&mut OsRng).to_string();
    let salt = salt_str.as_bytes();

    // hash password
    let mut output_key_material = vec![0u8; pwd.len()];
    Argon2::default().hash_password_into(pwd, salt, &mut output_key_material).unwrap(); // TODO: handle unwrap
    return output_key_material;
}

pub fn store(data: Vec<u8>) -> Result<(), Box<dyn Error>> {

    Ok(())
}

pub enum Type {
    Hash,
    Encryption,
    Nonce,
}

pub fn get(etype: Type) {
    // Match
}