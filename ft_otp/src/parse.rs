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

/*
 * Get position and which flags are used from a list a flags
 * then register the asked value
 */
pub fn  find_flags(_value: &String, g_flag: &mut String, k_flag: &mut String) {
    let (g_bool, g_pos) = parse_flags(_value, &"-g");
    let (k_bool, k_pos) = parse_flags(_value, &"-k");

    if g_bool {
        parse_value(_value, g_pos + 2, g_flag);
    }
    if k_bool {
        parse_value(_value, k_pos + 2, k_flag);
    }
}