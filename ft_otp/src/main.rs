use std::env;
use std::fs::File;
use std::io::Read;
use keyring::Entry;
use openssl::error::ErrorStack;
use openssl::hash::DigestBytes;

mod tools;
mod tools_encrypt;
mod tools_decrypt;
mod tools_keyrgs;
mod parse;

const FILENAME: &str = "ft_otp.key";
const ENCRYPTED_SIZE: usize = 256;
const UNCRYPTED_SIZE: usize = 248;

//https://datatracker.ietf.org/doc/html/rfc8017 encrypt public key
//https://datatracker.ietf.org/doc/html/rfc6238#page-2
//Must be TOTP but must be based on HOTP
//Check initial hexadecimal key K
//Generated code must be 6 digits using HMAC-SHA-1

//define T timer

//TOTP(K,T)
//Do something with T
//call hmac-sha-1 type crypt with K secret
//must generate 6 digits code

//extract argv part
//ask enter a secret
//get keyring secret
//hash secret
//cmp secret keyring
//-k open ft_otp.key
//if fail to open stop here
//decrypt ft_otp.key content
//decrypted key file content should be of at least 64 characters and max 256
//then call TOTP(K, T)

fn decrypt_file(k_flag: &String, digest: &DigestBytes) -> bool {
    let mut tmp = false;
    let mut buf: [u8; UNCRYPTED_SIZE] = [0; UNCRYPTED_SIZE];
    let res_file: Result<File, std::io::Error> = tools::open(k_flag);

    match res_file {
        Ok(mut file) => {
            let mut text_cipher: Vec<u8> = Vec::new();
            text_cipher.reserve(ENCRYPTED_SIZE);
            let res_size: Result<usize, std::io::Error> = file.read_to_end(&mut text_cipher);

            match res_size {
                Ok(size) => {
                    //if not 256 stop here
                    if size != ENCRYPTED_SIZE {
                        eprintln!("Encoded file should be of size {} bits", ENCRYPTED_SIZE);
                        return false;
                    }
                    println!("usize:{}",size);
                    text_cipher.resize(ENCRYPTED_SIZE, 0);
                    tmp = tools_decrypt::decrypt_aes(&digest, &mut buf, &text_cipher);
                    let aze = String::from_utf8(buf.to_ascii_uppercase()).unwrap();
                    println!("buf: {}", aze);
                },
                Err(e) => {eprintln!("Error: {e}")},
            }
        },
        Err(e) => {eprintln!("Error: {e}")},
    }
    tmp
}

/* Get keyring part */
fn keyring_comparison(k_flag: &String, input: &String) -> bool {
    let mut tmp: bool = false;
    let ret_str: &str;
    let res_secret: Result<openssl::hash::DigestBytes, ErrorStack>;
    let res_keyring: Result<Vec<u8>, keyring::Error>;

    let res_entry = tools_keyrgs::request_entry().unwrap();
    res_keyring = tools_keyrgs::get_keyring(&res_entry);
    match res_keyring {
        Ok(keyring) => {
            ret_str = input.trim_end();
            //hash secret
            res_secret = tools_encrypt::hash_secret(ret_str);
            match res_secret {
                Ok(digest) => {
                    let res: bool = tools_keyrgs::cmp_keyring(&digest, &keyring);

                    if !res {
                        return false;
                    }
                    tmp = decrypt_file(k_flag, &digest);
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                    return false;
                },
            }
        },
        Err(e) => {
            eprintln!("Error: {e}");
            eprintln!("Try to register a key first with ./ft_otp -g [Hexadecimal file].");
        },
    }
    return tmp;
}

fn password_generation(k_flag: &String) {
    //Enter a secret part
    let mut input: String = Default::default();
    let res_size = tools::get_input(&mut input);

    if let Ok(size) = res_size {
        println!("Input registered successfully: {}", size);
    } else {
        eprintln!("Couldn't get input");
        return ();
    }
    let is_decrypted = keyring_comparison(k_flag, &input);

    if is_decrypted {
        println!("gogo");
        //totp(K = buf, T)
    }
}

//-g
//get og key file
//create a file
//stop if can't create file   
//og key content must be full hexadecimal
//key.hex is uncrypted hex
//ft_otp.key is encrypted hex
//encrypt key using aes and a secret
//use keyring to save encrypted secret
//og key file content should be of at least 64 characters and max 256
//encrypted content must be stored in ft_otp.key
//must encrypt the key

//RUSTFLAGS=-Zsanitizer=leak RUSTFLAGS+=" -Zexport-executable-symbols" cargo +nightly run -Zbuild-std --target x86_64-unknown-linux-gnu
//try buf max 248

/* à nettoyer */
fn encode_part(buf: &Vec<u8>) {
    //ask for keyring
    let mut input: String = Default::default();
    let res_entry: Result<Entry, keyring::Error> = tools_keyrgs::request_entry();

    match res_entry {
        Ok(entry) => {
            let res: Result<Vec<u8>, keyring::Error> = tools_keyrgs::get_keyring(&entry);

            if let Err(e) = res {
                println!("{e}");
                let res_size = tools::get_input(&mut input);

                if let Ok(size) = res_size {
                    println!("Input registered successfully: {}", size);
                } else {
                    eprintln!("Couldn't get input");
                    return ();
                }
                let ret_str = input.trim_end();
                let res_digest: Result<openssl::hash::DigestBytes, ErrorStack> = tools_encrypt::hash_secret(&ret_str);

                match res_digest {
                    Ok(digest) => {
                        let res = tools_keyrgs::register_keyring(&entry, &digest);

                        //futur fonction
                        if res {
                            let mut text_cipher: [u8; ENCRYPTED_SIZE] = [0; ENCRYPTED_SIZE];

                            tools_encrypt::encrypt_aes(&digest, &mut text_cipher, &buf);
                            tools::file_new_and_write(&text_cipher, FILENAME);
                        }
                    },
                    Err(e) => {eprintln!("Error: {e}")},
                }
            } else {
                //ask if want to delete secret
                //si oui delete > delete secret, en fait un nouveau
                //call fonction encrypt
            }
        },
        Err(e) => {
            eprintln!("Error: {e}");
        },
    }
}

fn store_key(g_flag: &String) -> bool {
    let file: Result<File, std::io::Error> = tools::open(g_flag);
    let mut buf: Vec<u8> = Vec::new();

    buf.reserve(UNCRYPTED_SIZE);
println!("le:{}", buf.len());
    match file {
        Ok(mut f) => {
            let res_size: Result<usize, std::io::Error> = f.read_to_end(&mut buf);

            match res_size {
                Ok(size) => {
                    println!("SI:{}", size);
                    if size < 64 || size > UNCRYPTED_SIZE {
                        eprintln!("Error: key must be between 64 and {0} hexadecimal characters: {1}", UNCRYPTED_SIZE, size);
                        return false;
                    }
                    buf.resize(UNCRYPTED_SIZE, 0);
                    let txt: String = String::from_utf8(buf.clone()).expect("Something went wrong with private Key.");
println!("l:{}", txt.len());
                    let txt = txt.trim_end_matches('\0');
                    if !tools::regex_key(&txt) {
                        eprintln!("Key is invalid hex format");
                        return false;
                    }
                    encode_part(&buf);
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                },
            }
        },
        Err(e) => {
            eprintln!("Error: {e}");
            return  false;
        },
    }
    true
}

fn main() {
    let argv = env::args().skip(1);
    let mut concat_argv: String = String::from("");
    let mut g_flag: String = String::from("");
    let mut k_flag: String = String::from("");

    for i in argv {
        concat_argv.push_str(i.as_str());
        concat_argv.push(' ');
    }
    println!("flag:{concat_argv}");
    parse::find_flags(&concat_argv, &mut g_flag, &mut k_flag);
    println!("g:{g_flag} k:{k_flag}");
    if 0 < g_flag.len() {
        if !store_key(&g_flag) {
            return ();
        }
    }
    //check k_flag len if > 0 call ft
    if 0 < k_flag.len() {
        password_generation(&k_flag);
    }
}
