use std::env;
use std::fs::File;
use std::io::Read;
use keyring::Entry;
use openssl::error::ErrorStack;
use openssl::hash::DigestBytes;

mod define;
mod tools;
mod tools_encrypt;
mod tools_decrypt;
mod tools_keyrgs;
mod parse;
mod totp;

/*
 * Open and read ft_otp.key
 * Decrypt encrypted content from ft_otp.key
 * Trim eventuals \0
 * Call TOTP generation
 */
fn read_crypt(k_flag: &String, digest: &DigestBytes) -> bool {
    let mut buf: [u8; define::UNCRYPTED_SIZE] = [0; define::UNCRYPTED_SIZE];
    let res_file: Result<File, std::io::Error> = tools::open(k_flag);

    let ret: bool = match res_file {
        Ok(mut file) => {
            let mut text_cipher: Vec<u8> = Vec::new();
            text_cipher.reserve(define::ENCRYPTED_SIZE);
            let res_size: Result<usize, std::io::Error>
                = file.read_to_end(&mut text_cipher);

            let ret: bool = match res_size {
                Ok(size) => {
                    if size != define::ENCRYPTED_SIZE {
                        eprintln!("Encoded file should be of size {} bits",
                            define::ENCRYPTED_SIZE);
                        return false;
                    }
                    text_cipher.resize(define::ENCRYPTED_SIZE, 0);
                    let mut res: bool =
                                tools_decrypt::decrypt_bytes(digest, &mut buf, &text_cipher);
                    let res_hex_str = String::from_utf8(buf.to_vec());

                    match res_hex_str {
                        Ok(hex_str) => {
                            let hex_str = hex_str.trim_end_matches('\0');
                            if !tools::regex_key(&hex_str) {
                                eprintln!("Key is invalid hex format");
                                return false;
                            }
                            if res {
                                res = totp::start_totp(hex_str);
                            }
                        },
                        Err(_) => {
                            eprintln!("String buffer wrong format");
                            res = false;
                        }
                    }
                    
                    res
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                    false
                },
            };

            ret
        },
        Err(e) => {
            eprintln!("Error: {e}");
            false
        },
    };

    ret
}

/*
 * Get keyring part
 * Hash asked secret, compare with keyring hash
 * Call read_crypt function
 */
fn decrypt_comparaison(k_flag: &String, input: &String) -> bool {
    let ret_str: &str;
    let res_secret: Result<openssl::hash::DigestBytes, ErrorStack>;
    let res_keyring: Result<Vec<u8>, keyring::Error>;
    let res_entry = tools_keyrgs::request_entry();

    return match res_entry {
        Ok(entry) => {
            res_keyring = tools_keyrgs::get_keyring(&entry);
            return match res_keyring {
                Ok(keyring) => {
                    ret_str = input.trim_end();
                    res_secret = tools_encrypt::hash_secret(ret_str);
                    return match res_secret {
                        Ok(digest) => {
                            let res: bool
                                    = tools_keyrgs::cmp_keyring(&digest, &keyring);
        
                            if !res {
                                return false;
                            }
                            let ret: bool = read_crypt(k_flag, &digest);
        
                            ret
                        },
                        Err(e) => {
                            eprintln!("Error: {e}");
                            false
                        },
                    };
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                    eprintln!("Try to register a key first with ./ft_otp -g [Hexadecimal file].");
                    false
                },
            };
        },
        Err(e) => {
            eprintln!("Error: {e}");
            false
        },
    }
}

/*
 * Start of TOTP generation
 * Ask for secret
 * Ask keyring function
 */
fn secret_cmp_decrypt(k_flag: &String) -> bool {
    let mut input: String = Default::default();
    let res_size = tools::get_input(&mut input);

    if let Ok(size) = res_size {
        println!("Input registered successfully: {}", size);
    } else {
        eprintln!("Couldn't get input");
        return false;
    }
    let ret = decrypt_comparaison(k_flag, &input);

    ret
}

/*
 * End of encoding part
 * Get keyring, create one if needed
 * Store hashed secret in keyring
 * Write encoded key in file ft_otp.key
 */
fn end_encode_part(res_digest: &Result<DigestBytes, ErrorStack>,
                entry: &Entry, buf: &Vec<u8>, deleted: bool) -> bool {
    let mut res: bool = false;

    match res_digest {
        Ok(digest) => {
            if deleted {
                res = tools_keyrgs::register_keyring(&entry, &digest);
            } else {
                let res_keyring = tools_keyrgs::get_keyring(&entry);

                match res_keyring {
                    Ok(key) => {
                        res = tools_keyrgs::cmp_keyring(&digest, &key);
                    },
                    Err(e) => {
                        eprintln!("Error: {e}");
                        res = false;
                    },
                }
            }
            //futur fonction
            if res {
                let mut text_cipher: [u8; define::ENCRYPTED_SIZE]
                                    = [0; define::ENCRYPTED_SIZE];

                res = tools_encrypt::encrypt_aes(&digest, &mut text_cipher, &buf);
                if res {
                    res = tools::file_new_and_write(&text_cipher, define::FILENAME);
                }
            }
        },
        Err(e) => {eprintln!("Error: {e}")},
    }
    res
}

/*
 * Ask input and hash secret
 * Call end of hash/encryption part
 */
fn hash_input(entry: &Entry, buf: &Vec<u8>, deleted: bool) -> bool {
    let mut input: String = Default::default();
    let res_size = tools::get_input(&mut input);

    if let Ok(size) = res_size {
        println!("Input registered successfully: {}", size);
    } else {
        eprintln!("Couldn't get input");
        return false;
    }
    let ret_str = input.trim_end();
    let res_digest: Result<openssl::hash::DigestBytes, ErrorStack>
                   = tools_encrypt::hash_secret(&ret_str);
    let ret = end_encode_part(&res_digest, &entry, buf, deleted);

    ret
}

/* 
 * Get keyring from Linux system
 * Call hash secret input
 */
fn encode_part(buf: &Vec<u8>) -> bool {
    let res_entry: Result<Entry, keyring::Error>
                    = tools_keyrgs::request_entry();
    let ret = match res_entry {
        Ok(entry) => {
            let res: Result<Vec<u8>, keyring::Error>
                    = tools_keyrgs::get_keyring(&entry);

            if let Err(e) = res {
                println!("{e}");
                let ret: bool = hash_input(&entry, buf, true);

                ret
            } else {
                let mut ask: bool
                    = tools::ask_question("Do you want to delete existing secret?");
                if ask {
                    let _ = entry.delete_credential();
                    println!("Deleted successfully");
                    ask = hash_input(&entry, buf, true);
                } else {
                    ask = hash_input(&entry, buf, false);
                }
                ask
            }
        },
        Err(e) => {
            eprintln!("Error: {e}");
            false
        },
    };

    ret
}

/*
 * -g flag
 * Read asked file
 * Check if file content is valid
 * Store encrypted key of 256bytes size
 * Create a file where the encrypted key will be stored
 */
fn encrypt_key(g_flag: &String) -> bool {
    let file: Result<File, std::io::Error> = tools::open(g_flag);
    let mut buf: Vec<u8> = Vec::new();

    buf.reserve(define::UNCRYPTED_SIZE);
    let res: bool = match file {
        Ok(mut f) => {
            let res_size: Result<usize, std::io::Error> = f.read_to_end(&mut buf);
            let res = match res_size {
                Ok(size) => {
                    if size < 64 || size > define::UNCRYPTED_SIZE {
                        eprintln!("Error: key must be between 64 and {0} hexadecimal characters: {1}",
                                    define::UNCRYPTED_SIZE, size);
                        return false;
                    }
                    buf.resize(define::UNCRYPTED_SIZE, 0);
                    let txt: String = String::from_utf8(buf.clone())
                                    .expect("Something went wrong with private Key.");
                    let txt = txt.trim_end_matches('\0');

                    if !tools::regex_key(&txt) {
                        eprintln!("Key is invalid hex format");
                        return false;
                    }
                    let ret = encode_part(&buf);

                    ret
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                    false
                },
            };

            res
        },
        Err(e) => {
            eprintln!("Error: {e}");
            return  false;
        },
    };
    res
}

//RUSTFLAGS=-Zsanitizer=leak RUSTFLAGS+=" -Zexport-executable-symbols" cargo +nightly run -Zbuild-std --target x86_64-unknown-linux-gnu
//try buf max 248
/*
 * Rfc 4226 : https://datatracker.ietf.org/doc/html/rfc4226
 * Rfc 6238 : https://datatracker.ietf.org/doc/html/rfc6238
 * Init -g and -k flags
 * Create encrypted file if asked
 * Show TOTP password if asked
 */
fn main() -> Result<(), String> {
    let argv = env::args().skip(1);
    let mut concat_argv: String = String::from("");
    let mut g_flag: String = String::from("");
    let mut k_flag: String = String::from("");

    for i in argv {
        concat_argv.push_str(i.as_str());
        concat_argv.push(' ');
    }
    parse::find_flags(&concat_argv, &mut g_flag, &mut k_flag);
    if 0 < g_flag.len() {
        if !encrypt_key(&g_flag) {
            return Err("Couldn't encrypt key".to_owned());
        }
    }
    if 0 < k_flag.len() {
        if !secret_cmp_decrypt(&k_flag) {
            return Err("Couldn't generate TOTP code.".to_owned());
        }
    }
    Ok(())
}
