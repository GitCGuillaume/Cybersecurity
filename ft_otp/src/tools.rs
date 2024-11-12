use std::{fs::File, io::Write};
use regex::Regex;
use ring:: rsa::KeyPair;

/*
 *   https://fr.wikipedia.org/wiki/Public_Key_Cryptographic_Standards
 */
pub fn encrypt_pkcs_rsa(buf: &[u8]) -> Result<KeyPair, ring::error::KeyRejected>{
    let res: Result<KeyPair, ring::error::KeyRejected> = KeyPair::from_der(buf);

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
    let regex: Result<Regex, regex::Error> = Regex::new("^[a-fA-F0-9].+");

    let res: bool = match regex {
        Ok(reg) => {
            let captures = reg.captures(value);
            let res: bool = match captures {
                Some(capture) => {
                    let get_str = capture.get(0)
                                    .expect("Error while capturing key format.").as_str();
                    if get_str == "Error while capturing key format." {
                        eprintln!("{get_str}");
                        return false;
                    }
                    return true;
                },
                None => {
                    eprintln!("Private key must be of hexadecimal format.");
                    false
                },
            };
            res
        },
        Err(e) => {
            eprintln!("Error: {e}");
            false
        },
    };
    res
}