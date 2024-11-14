use std::env;
use std::fs::File;
use std::io::Read;
use openssl::{
    pkey::{Private, PKey},
    error::ErrorStack
};

mod tools;
mod parse;
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
//open a file
//stop if can't open file   
//og key content must be full hexadecimal
//key.hex is private
//ft_otp.key is secret (public)
//public to encrypt
//private to decrypt
//og key file content should be of at least 64 characters
//og key content must be stored in ft_otp.key
//must encrypt the key

//RUSTFLAGS=-Zsanitizer=leak RUSTFLAGS+=" -Zexport-executable-symbols" cargo +nightly run -Zbuild-std --target x86_64-unknown-linux-gnu

fn store_key(g_flag: &String) -> bool {
    let file: Result<File, std::io::Error> = tools::open(g_flag);
    let mut buf: Vec<u8> = Vec::new();

    match file {
        Ok(mut f) => {
            let res_size: Result<usize, std::io::Error> = f.read_to_end(&mut buf);

            match res_size {
                Ok(size) => {
                    let txt: String = String::from_utf8(buf.clone()).expect("Something went wrong with private Key.");

                    if txt.len() <= 64 {
                        eprintln!("Error: key must be at least 64 hexadecimal characters: {0}", size);
                        return false;
                    }
                    if !tools::regex_key(&txt) {
                        eprintln!("Key is invalid hex format");
                        return false;
                    }
                    let res_pkey: Result<PKey<Private>, ErrorStack> = tools::generate_rsa();

                    match res_pkey {
                        Ok(pkey) => {
                            let res_buf: Result<Vec<u8>, ErrorStack> = tools::encrypt_data(&pkey, &buf);

                            match res_buf {
                                Ok(buf_enc) => {
                                    tools::file_new_and_write(&buf_enc, "ft_otp.key");
                                },
                                Err(_) => {},
                            }
                        },
                        Err(e) => {eprintln!("Error: {e}")},
                    }
                    //let res_encrypt: Result<KeyPair, KeyRejected> = tools::encrypt_pkcs_rsa(&buf);

                    /*match res_encrypt {
                        Ok(encrypt) => {dbg!(encrypt);},
                        Err(e) => {eprintln!("Error: {e}")},
                    }*/
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                },
            }
            //encrypt secret key from og hexa value
            //tools::encrypt_pkcs_rsa(&f);
            //register
            //tools::file_new_and_write(&f, "ft_otp.key");
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
