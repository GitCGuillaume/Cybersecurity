mod tools;
mod tests;
mod encrypt;
mod decrypt;
use std::process::ExitCode;
use tools::{Flags, get_flags, set_key};

fn run_commands_flags(list: &Flags) -> bool {
    if list.help {
        println!("-h or --help to show help.");
        println!("-v or --version to show version");
        println!("-s or --silent to not produce output.");
        println!("-r or --reverse followed by key argument, \
            to reverse infection.");
        return true;
    }
    if list.version {
        println!("Version Stockholm: 1.0");
        return true;
    }
    false
}


/*
 * Start program
 * Set flags
 * Check key length
 * Choose between encryption and decryption
 */
fn main() -> ExitCode {
    let args: std::iter::Skip<std::env::Args> = std::env::args().skip(1);
    let mut list = Flags {
        key: String::default(),
        reverse_key: String::default(),
        help: false,
        version: false,
        silent: false
    };
    if args.len() == 0 {
        return ExitCode::SUCCESS;
    }
    let mut value: String = String::new();
    for i in args {
        if i == "-r" {
            value = i.to_owned();
            continue;
        } else if 0 < value.len() {
            value.push_str(i.as_str());
        } else {
            value.clear();
            value = i.to_owned();
        }
        get_flags(&value, &mut list);
        set_key(&value, &mut list);
        value.clear();
    }
    if run_commands_flags(&list) {
        return ExitCode::SUCCESS;
    }
    if list.reverse_key.len() != 0 {
        if list.reverse_key.len() != 16 {
            eprintln!("Key must be of size 16 characters: {}",
                list.reverse_key.len());
            return ExitCode::FAILURE;
        }
        if !decrypt::start_decrypt(&list) {
            return ExitCode::FAILURE;
        }
    } else {
        if list.key.len() != 16 {
            eprintln!("Key must be of size 16 characters.: {}",
                list.key.len());
            return ExitCode::FAILURE;
        }
        if !encrypt::start_encrypt(&list) {
            return ExitCode::FAILURE;
        }
    }
    return ExitCode::SUCCESS;
}
