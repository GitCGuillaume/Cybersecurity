/* 
 * Trim spaces and removes useless values
 */
fn parse_value(_value: &String, _pos: usize, _fill_string: &mut String) {
    let mut _tmp: String = String::from(_value);

    _tmp.replace_range(.._pos, "");
    let _str: &str = _tmp.trim_start();
    let _space_pos: Option<usize> = _str.find(" ");

    _fill_string.clear();
    _fill_string.insert_str(0, _str);
    match _space_pos {
        Some(pos) => {
            _fill_string.replace_range(pos.., "");
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
        pub folder: String
    }

    /*
    * Get position and which flags are used from a list a flags
    * then register the asked value
    */
    pub fn  find_flags(_value: &String, mut _is_recursive: bool,
        _max_depth: &mut String, _path: &mut String) {
        let (_rl_bool, _rl_pos) = parse_flags::parse_flags(_value, &"-rl");
        let (_rp_bool, _rp_pos) = parse_flags::parse_flags(_value, &"-rp");
        let (_l_bool, _l_pos) = parse_flags::parse_flags(_value, &"-l");
        let (_p_bool, _p_pos) = parse_flags::parse_flags(_value, &"-p");

        if _rl_bool || _l_bool {
            if _l_pos <= _rl_pos && _rl_bool {
                _is_recursive = true;
                parse_flags::parse_value(_value, _rl_pos + 3, _max_depth);
            } else {
                parse_flags::parse_value(_value, _l_pos + 2, _max_depth);
            }
        }
        if _rp_bool || _p_bool {
            if _p_pos <= _rp_pos && _rp_bool {
                parse_flags::parse_value(_value, _rp_pos + 3, _path);
            } else {
                parse_flags::parse_value(_value, _p_pos + 2, _path);
            }
        }
    }
}