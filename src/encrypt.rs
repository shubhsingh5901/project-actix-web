use std::error::Error;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn encrypt_string(data: String, password: String) -> Result<String, Box<dyn Error>> {
    let mc = new_magic_crypt!(&password, 256);

    let encrypted_data = mc.encrypt_bytes_to_base64(data.as_bytes());
    Ok(encrypted_data)
}

pub fn decrypt_string(text: String, password: String) -> Result<String, Box<dyn Error>> {
    let mc = new_magic_crypt!(&password, 256);
    let data = mc.decrypt_base64_to_string(text)?;
    Ok(data)
}
