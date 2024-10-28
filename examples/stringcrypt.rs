use magic_crypt::{new_magic_crypt, MagicCryptTrait};
fn main() {
    let mc = new_magic_crypt!("magickey", 256);

    let base64 = mc.encrypt_str_to_base64("http://magiclen.org");
    println!("encrypted text {:?}", base64);

    let decypt_text = mc.decrypt_base64_to_string(&base64).unwrap();
    println!("decrypt text {:?}", decypt_text);
}
