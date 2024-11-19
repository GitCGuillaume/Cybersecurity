use std::{
    fs::File,
    io::{
        stdin, Write
    }
};
use regex::Regex;
use openssl::{
    aes,
    error::ErrorStack,
    hash,
};

pub fn encrypt_aes(secret: &[u8], text_cipher: &mut [u8; 256], buf: &Vec<u8>) {
    let encrypter = aes::AesKey::new_encrypt(secret).unwrap();
    
    println!("out:{0} in:{1}", text_cipher.len(), buf.len());
    let res= aes::wrap_key(&encrypter, None, text_cipher, buf).unwrap();
    println!("usize: {res}");
    let str1 = text_cipher.escape_ascii().to_string();
    let str2 = secret.escape_ascii().to_string();
    println!("cipher:{0}\n secret:{1}", str1, str2);
   //wrap_key(secret_key, none, données chiffrées, hex à chiffrer)
}

pub fn hash_str(input_str: &str) -> Result<hash::DigestBytes, ErrorStack> {
    let res_hasher: Result<hash::Hasher, ErrorStack> = hash::Hasher::new(hash::MessageDigest::sha256());

    let res: Result<hash::DigestBytes, ErrorStack> = match res_hasher {
        Ok(mut _hasher) => {
            let res_update: Result<(), ErrorStack> = _hasher.update(input_str.as_bytes());

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

pub fn get_input(input: &mut String ) -> Result<usize, std::io::Error> {
    println!("Please enter a secret:");
    let res_count: Result<usize, std::io::Error> = stdin().read_line(input);

    res_count
}

/* Try to create file then write in */
pub fn file_new_and_write(content: &[u8; 256], name: &str) {
    let res_file: Result<File, std::io::Error> = File::create_new(name);

    match res_file {
        Ok(mut file) => {
            let res_buf: Result<(), std::io::Error> = file.write_all(content);

            match res_buf {
                Ok(_) => {
                    println!("Key was successfully saved in {0}", name);
                },
                Err(e) => {eprintln!("Error: {e}")},
            }
        },
        Err(e) => {
            eprintln!("Error: {e}");
        },
    }
}

/* Open then read file */
pub fn open(g_flag: &String) -> Result<File, std::io::Error> {
    return File::open(g_flag);
}

/* Hexa Regex checker */
pub fn regex_key(value: &str) -> bool {
    println!("value:{value}");
    for c in value.chars() {
        println!("{c:?}");
    }
    let regex: Result<Regex, regex::Error> = Regex::new("^(?m)[a-fA-F0-9]+$").map(|f|f);
    let res: bool = match regex {
        Ok(reg) => {
            let captures = reg.split(value);
            for i in captures {
                if i.len() != 0 {
                    return  false;
                }
            }
            true
        },
        Err(e) => {
            eprintln!("Error: {e}");
            false
        },
    };
    res
}