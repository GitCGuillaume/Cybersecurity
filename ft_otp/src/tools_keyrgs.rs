use keyring::Entry;

pub fn cmp_keyring(secret: &[u8], secret_krng: &Vec<u8>) -> bool {
    let res: bool = secret == secret_krng;

    println!("secret cmp: {res}");
    if res {
        println!("Secret is correct!");
    } else {
        println!("Wrong secret! please retry.");
    }
    res
}

fn get_secret_keyring(entry: &Entry) -> Result<Vec<u8>, keyring::Error> {
    let ret_secret: Result<Vec<u8>, keyring::Error> = entry.get_secret();

    ret_secret
}



/*
    Trying to get keyring, otherwise create it
*/
pub fn request_entry() -> Result<Entry, keyring::Error> {
    let username: String = whoami::username();
    let entry: Result<Entry, keyring::Error> = Entry::new("ft_otp", &username);

    entry
}

//ask for keyring
//if no keyring ask user to enter a secret
pub fn register_keyring(entry: &Entry, secret: &[u8]) -> bool {
    let res = entry.set_secret(secret);
    let res = match res {
        Ok(_) => {
            println!("Secret registered successfully");
            true
        },
        Err(e) => {
            eprintln!("Error: {e}");
            false
        },
    };

    res
}

/* Get a keyring using username */
pub fn get_keyring(entry: &Entry) -> Result<Vec<u8>, keyring::Error> {
    let res_secret = get_secret_keyring(entry);

    res_secret
}