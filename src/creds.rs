use argon2::Argon2;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

pub fn encrypt(data: String) {
    // gen nonce
    // encrypt with master pass and nonce
    
    //store(data, Type::Encryption);
}

pub fn decrypt(data: String) {

}

pub fn hash(pass: String) -> Result<Vec<u8>, Box<dyn Error>> {
    // gen salt
    let pwd = pass.as_bytes();
    let salt_str = SaltString::generate(&mut OsRng).to_string();
    let salt = salt_str.as_bytes();

    // hash password
    let mut output_key_material = vec![0u8; pwd.len()];
    match Argon2::default().hash_password_into(pwd, salt, &mut output_key_material) { // TODO: handle unwrap
        Ok(_) => Ok(output_key_material),
        Err(e) => Err(format!("Hash failed: {}", e.to_string()).into())
    }
}

pub fn store(data: Vec<u8>, etype: Type) -> Result<(), Box<dyn Error>> {
    let data_str: String= data.iter().map(|&byte| byte as char).collect();
    
    let mut file = match etype {
        Type::Hash => OpenOptions::new().write(true).append(false).create(false).open("./bin.txt")?,
        Type::Encryption => OpenOptions::new().write(true).append(true).create(true).open("./bin.txt")?
    };
    
    file.write_all(data_str.as_bytes())?;

    Ok(())
}

pub fn remove(num: String) {

}

pub enum Type {
    Hash,
    Encryption
}

pub fn get_hash() {
    
}