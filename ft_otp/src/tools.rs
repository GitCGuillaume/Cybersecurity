use std::{ fs::File, io::Write };
use regex::Regex;
use openssl::{
    encrypt::Encrypter,
    error::ErrorStack,
    pkey::{ PKey, Private },
    rsa::{ Padding, Rsa }
};
use keyring::{
    default::{self, default_credential_builder}, set_default_credential_builder, CredentialBuilder, Entry
};


//ok so now ask to enter a secret if no keyring found
//use this secret to generate something to encrypt?
fn generate_pkey_pair(pkey: &Rsa<Private>) {
    let res_public = pkey.public_key_to_der();
    //testing panic on purpose
    /*let def: Box<dyn keyring::credential::CredentialBuilderApi + Send + Sync> = default_credential_builder();
    //let res_build: Result<Box<dyn keyring::credential::CredentialApi + Send + Sync>, keyring::Error>
      //  = def.build(Some("aze"), "linux-native", "guillaume");
    match res_build {
        Ok(build) => {
            set_default_credential_builder(build);
        },
        Err(_) => {},
    }
    def.*/
    //let aze = CredentialBuilder::build(Some("aze"), "linux-native", "guillaume");
    //set_default_credential_builder(def);
    let username = whoami::username();
    println!("u:{username}");
    let entry = Entry::new("ft_otp", &username).unwrap();
    let persistence = default::default_credential_builder().persistence();

    if  matches!(persistence, keyring::credential::CredentialPersistence::UntilDelete) {
        println!("The default credential builder persists credentials on disk!")
    } else {
        println!("The default credential builder doesn't persist credentials on disk!")
    }
    match res_public {
        Ok(public) => {
            /*let res = entry.set_password("aze");
            match res {
                Ok(r) => {},
                Err(e) => {eprintln!("Error public: {e}")},
            }*/
            /*let res = entry.set_secret(&public);
            match res {
                Ok(r) => {},
                Err(e) => {eprintln!("Error public: {e}")},
            }*/
            let aa = entry.get_secret().unwrap();
            
            //let aze = entry.get_password().unwrap();
            println!("{:?}", aa);
        },
        Err(e) => {eprintln!("Error: {e}")},
    }
    /*let res_private = pkey.private_key_to_der();

    match res_private {
        Ok(private) => {
            let res = entry.set_secret(&private);
            match res {
                Ok(r) => {},
                Err(e) => {eprintln!("Error private: {e}")},
            }
        },
        Err(e) => {eprintln!("Error: {e}")},
    }*/
   // dbg!(entry);
}

/* Encrypter need a (public) pkey  */
pub fn generate_rsa() -> Result<PKey<Private>, ErrorStack> {
    let res_rsa: Result<Rsa<Private>, ErrorStack> = Rsa::generate(2048);

    let rsa: Result<PKey<Private>, ErrorStack> = match res_rsa {
        Ok(rsa) => {
            let res: Result<PKey<Private>, ErrorStack> = PKey::from_rsa(rsa);

            res
        },
        Err(e) => {
            eprintln!("Error: {e}");
            Err(e)
        },
    };
    rsa
}

/*
 * Random length padding is primarily used to prevent attackers
 * from predicting or knowing the exact length of a plaintext message
 * that can possibly lead to breaking encryption.
 * Source: https://docs.rs/openssl/latest/openssl/rsa/struct.Padding.html#associatedconstant.PKCS1
 */
pub fn encrypt_data(pkey: &PKey<Private>, buf: &Vec<u8>) -> Result<Vec<u8>, ErrorStack> {
    let aze = pkey.rsa().unwrap();
    generate_pkey_pair(&aze);
    //let aa = aze.public_key_to_pem();
    //dbg!(String::from_utf8(aa.unwrap()));
    //let bb = aze.private_key_to_pem();
    //dbg!(String::from_utf8(bb.unwrap()));
    let res = Encrypter::new(pkey);

    let res: Result<Vec<u8>, ErrorStack> = match res {
        Ok(mut encrypter) => {
            let mode = encrypter.set_rsa_padding(Padding::PKCS1);

            match mode {
                Ok(_) => {},
                Err(e) => {eprintln!("Error: {e}")},
            }
            let len = encrypter.encrypt_len(buf);
            let len = len.unwrap();
            println!("len: {}", len);
            let mut buf_encrypt = vec![0; len];
            let size = encrypter.encrypt(buf, &mut buf_encrypt);
            buf_encrypt.truncate(len);
            println!("{}", size.unwrap());
            Ok(buf_encrypt)
        },
        Err(e) => {
            eprintln!("Error: {e}");
            Err(e)
        },
    };
    res
}

//???
//gpg --list-public-keys
//Display credential status reboot
fn init_keyring() {
}

fn add_keyring() {
    
}

//???
fn request_keyring() {

}

/* Try to create file then write in */
pub fn file_new_and_write(content: &Vec<u8>, name: &str) {
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
    let regex: Result<Regex, regex::Error> = Regex::new(r"^(?m)[a-fA-F0-9]+$").map(|f|f);
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