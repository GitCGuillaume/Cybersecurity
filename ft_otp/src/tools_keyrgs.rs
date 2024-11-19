use keyring::Entry;

pub fn request_keyring(secret: &[u8]) -> bool {
    let username = whoami::username();
    let entry = Entry::new("ft_otp", &username).unwrap();
    let ret_secret = entry.get_secret();

    match ret_secret {
        Ok(secret_krng) => {
            let res = secret == secret_krng;

            println!("secret cmp: {res}");
            if res {
                println!("Secret is correct!");
            } else {
                println!("Wrong secret! please retry.");
            }
            res
        },
        Err(_) => {
            let res = entry.set_secret(secret);
            let res = match res {
                Ok(_) => true,
                Err(e) => {
                    eprintln!("Error: {e}");
                    false
                },
            };
            res
        }
    }
}