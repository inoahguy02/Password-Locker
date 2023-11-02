use std::error::Error;

pub fn encrypt(data: String) {

}

pub fn decrypt(data: String) {

}

pub fn hash(pass: String) -> String {
    // gen salt
    // hash password
    return pass; // this is just here to get rid of error
}

pub fn store(data: String) -> Result<(), Box<dyn Error>> {

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