/* 
 * Trim spaces and removes useless values
 */
fn parse_value(value: &String, pos: usize, fill_string: &mut String) {
    let mut _tmp: String = String::from(value);

    _tmp.replace_range(..pos, "");
    let str: &str = _tmp.trim_start();
    let space_pos: Option<usize> = str.find(" ");

    fill_string.clear();
    fill_string.insert_str(0, str);
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

pub mod parse {
    use crate::parse_flags;
    pub struct OptionUser {
        pub url: String,
        pub folder: String,
        pub website_name: String
    }

    /*
    * Get position and which flags are used from a list a flags
    * then register the asked value
    */
    pub fn  find_flags(value: &String, mut _is_recursive: bool,
        max_depth: &mut String, path: &mut String) {
        let (rl_bool, rl_pos) = parse_flags::parse_flags(value, &"-rl");
        let (rp_bool, rp_pos) = parse_flags::parse_flags(value, &"-rp");
        let (l_bool, l_pos) = parse_flags::parse_flags(value, &"-l");
        let (p_bool, p_pos) = parse_flags::parse_flags(value, &"-p");

        if rl_bool || l_bool {
            if l_pos <= rl_pos && rl_bool {
                _is_recursive = true;
                parse_flags::parse_value(value, rl_pos + 3, max_depth);
            } else {
                parse_flags::parse_value(value, l_pos + 2, max_depth);
            }
        }
        if rp_bool || p_bool {
            if p_pos <= rp_pos && rp_bool {
                parse_flags::parse_value(value, rp_pos + 3, path);
            } else {
                parse_flags::parse_value(value, p_pos + 2, path);
            }
        }
    }
}