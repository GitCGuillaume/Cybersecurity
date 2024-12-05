use openssl::{
    aes,
    error::ErrorStack,
    hash,
};
use crate::define;

/*
 * Encrypt Hexa key
 */
pub fn encrypt_aes(secret: &[u8],
    b_out: &mut [u8; define::ENCRYPTED_SIZE],
    b_in: &Vec<u8>)
    -> bool {
    let res_encrypter = aes::AesKey::new_encrypt(secret);

    if let Ok(encrypter) = res_encrypter {
        let res= aes::wrap_key(&encrypter, None, b_out, b_in);

        if let Err(_) = res {
            eprintln!("Error: Couldn't wrap secret key");
            return false;
        }
    } else {
        eprintln!("Error: Couldn't encrypt secret.");
        return false;
    }
    return true;
}

/*
 * Hash secret
 */
pub fn hash_secret(input_str: &str) -> Result<hash::DigestBytes, ErrorStack> {
    let res_hasher: Result<hash::Hasher, ErrorStack>
                    = hash::Hasher::new(hash::MessageDigest::sha256());

    let res: Result<hash::DigestBytes, ErrorStack> = match res_hasher {
        Ok(mut _hasher) => {
            let res_update: Result<(), ErrorStack>
                            = _hasher.update(input_str.as_bytes());

            let res = match res_update {
                Ok(_) => {
                    let res_finish = _hasher.finish();

                    res_finish
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                    Err(e)
                },
            };
            res
        },
        Err(e) => {
            eprintln!("Error: {e}");
            Err(e)
        },
    };
    res
}