use openssl::{
    aes::{
        self, unwrap_key
    },
    hash::DigestBytes
};
use crate::define;

/*
 * Decrypt ft_otp.key in AES format
 */
fn decrypt_aes(secret: &[u8],
    b_out: &mut [u8; define::UNCRYPTED_SIZE],
    b_in: &Vec<u8>)
    -> bool {
    let decrypter = aes::AesKey::new_decrypt(secret);

    if let Ok(res) = decrypter {
        let res = unwrap_key(&res, None, b_out, b_in);

        if let Err(_) = res {
            eprintln!("Error: Couldn't unwrap file");
            eprintln!("Do not modify the file!");
            eprintln!("Please make a new encrypted file with \
             ./ft_otp -g [Hexadecimal file]");
            return false;
        }
    } else {
        eprintln!("Error: Couldn't decrypt secret.");
        return false;
    }
    true
}

pub fn decrypt_bytes(digest: &DigestBytes,
    buf: &mut [u8; define::UNCRYPTED_SIZE],
    text_cipher: &Vec<u8>)
    -> bool {
    let tmp: bool = decrypt_aes(&digest, buf, text_cipher);

    if !tmp {
        return tmp;
    }
    return true;
}