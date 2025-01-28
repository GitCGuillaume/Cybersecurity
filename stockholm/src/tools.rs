pub mod parse;

pub struct Flags {
    pub key: String,
    pub reverse_key: String,
    pub help: bool,
    pub version: bool,
    pub silent: bool
}

/* Find starting position from flags */
fn  parse_flags(value: &String, flag: &str) -> bool {
    let first_pos: Option<usize> = value.rfind(flag);
    
    return match first_pos {
        Some(_) => {
            true
        },
        None => false
    };
}

/*
 * Try to find flags that need a boolean
 */
fn find_flags(value: &String, list: &mut Flags) {
    if parse_flags(value, "--help") {
        list.help = true;
    }
    if parse_flags(value, "-h") {
        list.help = true;
    }
    if parse_flags(value, "--version") {
        list.version = true;
    }
    if parse_flags(value, "-v") {
        list.version = true;
    }
    if parse_flags(value, "--silent") {
        list.silent = true;
    }
    if parse_flags(value, "-s") {
        list.silent = true;
    }
}

pub fn set_key(value: &String, list: &mut Flags) {
    let res_find = value.find("-");

    match res_find {
        Some(_) => {
        },
        None => {
            list.key = String::from(value);
        }
    }
}

/*
 * Set flags
 * Try to set reverse_key if needed
 */
pub fn get_flags(value: &String, list: &mut Flags) {
    find_flags(value, list);
    parse::find_reverse_key(value, list);
}
