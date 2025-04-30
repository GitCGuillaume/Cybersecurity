use std::path::PathBuf;
use std::fs::rename;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{
    Read,
    Write
};
use openssl::symm::{decrypt, Cipher};
use crate::tools::Flags;

/* Does not work on different mountpoint, see rename doc */
pub fn rename_desinfect(path: &PathBuf) {
    if path.is_file() {
        let res_path = path.to_str();

        if let Some(path) = res_path {
            if let Some(idx) = path.rfind(".ft") {
                if let Some(new_path) = path.get(0..idx) {
                    let _ = rename(path, new_path);
                }
            }
        }
    }
}

/*
 * Check key length and iv length
 * get IV
 * decrypt with AES
 * write decrypted content to file
 */

pub fn aes_desinfection(list: &Flags, buf_in: &mut Vec<u8>, file: &mut File) -> bool {
    let cipher_ecb = Cipher::aes_128_cbc();
    let key_bytes: &[u8] = list.reverse_key.as_bytes();

    if buf_in.len() < 16 {
        return false;
    }
    let start_len = buf_in.len() - 16;
    let mut iv: [u8; 16] = [0u8; 16];

    if key_bytes.len() != 16{
        eprintln!("key length must be of 16 bytes.");
        return false;
    }
    iv.clone_from_slice(&buf_in[start_len..]);
    for _ in 0..16 {
        buf_in.pop();
    }
    if key_bytes.len() != iv.len() {
        eprintln!("Key and IV length must be equal.");
        return false;
    }
    let res_buf_out = decrypt(cipher_ecb, key_bytes, Some(&iv), &buf_in);

    return match res_buf_out {
        Ok(buf) => {
            if let Err(e) = file.set_len(0) {
                eprintln!("{e}");
                return false;
            }
            if let Err(e) = file.write_all(&buf) {
                eprintln!("{e}");
                return false;
            }
            if let Err(e) = file.sync_data() {
                eprintln!("{e}");
                return false;
            }
            true
        },
        Err(e) => {
            println!("Error {e}");
            false
        }
    }
}

/*
 * Open file with rights
 * Run desinfect
 * Removee .ft extension
 */
pub fn desinfect(list: &Flags, path: &PathBuf) -> bool {
    let result_panic = std::panic::catch_unwind(||{
    let mut buf_in: Vec<u8> = Vec::new();
    if let Some(path_str) = path.to_str() {
        if let Ok(mut file) = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .append(true)
                                .create(false)
        .open(path_str){
            if let Ok(_) = file.read_to_end(&mut buf_in) {
                let res: bool = aes_desinfection(list, &mut buf_in, &mut file);
                if res {
                    rename_desinfect(&path);
                }
            } else {
                eprintln!("Couldn't read file.");
            }
        }
    }
    });
    match result_panic {
        Ok(_) => true,
        Err(_) => false
    }
}
