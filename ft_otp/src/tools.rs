use std::{fs::File, io::Write};
use regex::Regex;
use ring:: rsa::KeyPair;

/*
 *   https://fr.wikipedia.org/wiki/Public_Key_Cryptographic_Standards
 */
pub fn encrypt_pkcs_rsa(buf: &Vec<u8>) -> Result<KeyPair, ring::error::KeyRejected>{
    let res: Result<KeyPair, ring::error::KeyRejected> = KeyPair::from_der(&buf);

    res
}

/* Try to create file then write in */
pub fn file_new_and_write(content: &String, name: &str) {
    let res_file: Result<File, std::io::Error> = File::create_new(name);

    match res_file {
        Ok(mut file) => {
            let res_buf: Result<(), std::io::Error> = file.write_all(content.as_bytes());

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
    let regex: Result<Regex, regex::Error> = Regex::new(r"^(?m)[a-fA-F0-9\s]+$").map(|f|f);
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