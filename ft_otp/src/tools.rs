use std::{
    fs::File,
    io::{
        stdin, Write
    }
};
use regex::Regex;
use crate::define;

pub fn ask_quesion(question: &str) -> bool {
    let mut buf: String = Default::default();

    println!("{}", question);
    println!("Type Y if yes");
    let res = stdin().read_line(&mut buf);

    if let Ok(_) = res {
        let _str = buf.trim_end();
        let _str = _str.to_lowercase();

        if _str.eq("y") {
            return true;
        }
    }
    return false;
}

pub fn get_input(input: &mut String ) -> Result<usize, std::io::Error> {
    println!("Please enter a secret:");
    let res_count: Result<usize, std::io::Error> = stdin().read_line(input);

    res_count
}

/* Try to create file then write in */
pub fn file_new_and_write(content: &[u8; define::ENCRYPTED_SIZE], name: &str) {
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
    /*for c in value.chars() {
        println!("{c:?}");
    }*/
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