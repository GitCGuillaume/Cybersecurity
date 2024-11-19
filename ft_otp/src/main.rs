use std::env;
use std::fs::File;
use std::io::Read;
use openssl::error::ErrorStack;

mod tools;
mod tools_keyrgs;
mod parse;

const FILENAME: &str = "ft_otp.key";
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

//-k open ft_otp.key as default, otherwise open asked path
//if fail to open stop here
//check ft_otp.key content
//og key file content should be of at least 64 characters
//then call TOTP(K, T)

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
fn store_key(g_flag: &String) -> bool {
    let file: Result<File, std::io::Error> = tools::open(g_flag);
    let mut buf: Vec<u8> = Vec::new();
    buf.reserve(248);
println!("le:{}", buf.len());
    match file {
        Ok(mut f) => {
            let res_size: Result<usize, std::io::Error> = f.read_to_end(&mut buf);

            match res_size {
                Ok(size) => {
                    println!("SI:{}", size);
                    if size < 64 || size > 248 {
                        eprintln!("Error: key must be between 64 and 248 hexadecimal characters: {0}", size);
                        return false;
                    }
                    buf.resize(248, 0);
                    let txt: String = String::from_utf8(buf.clone()).expect("Something went wrong with private Key.");
println!("l:{}", txt.len());
                    let txt = txt.trim_end_matches('\0');
                    if !tools::regex_key(&txt) {
                        eprintln!("Key is invalid hex format");
                        return false;
                    }
                    let mut input: String = Default::default();

                    let res = tools::get_input(&mut input);

                    match res {
                        Ok(count) => {
                            if 0 < count {
                                let ret_str = input.trim_end();
                                let res_digest: Result<openssl::hash::DigestBytes, ErrorStack> = tools::hash_str(&ret_str);
                                match res_digest {
                                    Ok(digest) => {
                                        //keyring part
                                        let res = tools_keyrgs::request_keyring(&digest);

                                        if res {
                                            let mut text_cipher: [u8; 256] = [0u8; 256];
                                            tools::encrypt_aes(&digest, &mut text_cipher, &buf);
                                            tools::file_new_and_write(&text_cipher, FILENAME);
                                        }
                                    },
                                    Err(e) => {eprintln!("Error: {e}")},
                                }
                            } else {
                                eprintln!("Error: No secret provided.");
                            }
                        },
                        Err(e) => {eprintln!("Error: {e}")},
                    }
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
    }
}
