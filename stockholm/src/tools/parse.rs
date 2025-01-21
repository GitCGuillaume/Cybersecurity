use crate::tools::Flags;

/* 
 * Trim spaces and removes useless values
 */
fn parse_value(value: &String, pos: usize, fill_string: &mut String) {
    let mut tmp: String = String::from(value);

    tmp.replace_range(..pos, "");
    let trim_str: &str = tmp.trim_start();
    let space_pos: Option<usize> = trim_str.find(" ");

    fill_string.clear();
    fill_string.insert_str(0, trim_str);
    match space_pos {
        Some(pos) => {
            fill_string.replace_range(pos.., "");
        },
        None => {},
    }
}

/* Find starting position from flags */
fn  parse_flags(value: &String, flag: &str) -> (bool, usize) {
    let first_pos: Option<usize> = value.rfind(flag);
    
    match first_pos {
        Some(pos) => {
            let mut _str: &str = &value[pos..];

            return (true, pos);
        },
        None => return (false, 0)
    };
}

/*
 * Get position and which flags are used from a list a flags
 * Check "farthest value"
 * add +1 or +2 to pos because of start position being at first -
 * then register the asked value
 */
pub fn  find_reverse_key(value: &String, list: &mut Flags) {
    let (r_bool, r_pos) = parse_flags(value, &"-r");
    let (sr_bool, sr_pos) = parse_flags(value, &"-sr");
    let (rev_bool, rev_pos) = parse_flags(value, &"--reverse");

    if r_bool || sr_bool || rev_bool {
        let mut pos: usize = r_pos;
        let mut which_bool: u8 = 1;

        list.reverse_key.clear();
        if pos < sr_pos + 1 && sr_bool {
            pos = sr_pos;
            which_bool = 2;
        }
        if pos < rev_pos + 2 && rev_bool {
            which_bool = 3;
        }
        if which_bool == 1 {
                parse_value(value, r_pos + 2, &mut list.reverse_key);
        } else if which_bool == 2 {
                parse_value(value, sr_pos + 3, &mut list.reverse_key);
        } else {
                parse_value(value, rev_pos + 9, &mut list.reverse_key);
        }
    }
}
