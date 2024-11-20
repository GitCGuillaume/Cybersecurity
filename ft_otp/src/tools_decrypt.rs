use openssl::aes::{self, unwrap_key};

//decrypt_aes
pub fn decrypt_aes(secret: &[u8], b_out: &mut [u8; 248], b_in: &Vec<u8>) -> bool {
    println!("out: {0} in: {1}",b_out.len(), b_in.len());
    let decrypter = aes::AesKey::new_decrypt(secret).unwrap();

    let res = unwrap_key(&decrypter, None, b_out, b_in).unwrap();
    println!("usize: {res}");
    true
}