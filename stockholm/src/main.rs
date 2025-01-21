mod tools;
mod tests;
mod encrypt;
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

fn run_decrypt() {

}

fn run_encrypt() {

}

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
    for i in args {
        get_flags(&i, &mut list);
        set_key(&i, &mut list);
    }
    println!("k:{0} rev:{1} h:{2} v:{3} s:{4}", list.key, list.reverse_key,
        list.help, list.version, list.silent);
    if run_commands_flags(&list) {
        return ExitCode::SUCCESS;
    }
    if list.reverse_key.len() != 0 {

    } else {

    }
    return ExitCode::SUCCESS;
}
