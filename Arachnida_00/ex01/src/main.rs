/* 
 * Trim spaces and removes useless values
 */
fn parse_value(value: &String, pos: usize, fill_string: &mut String) {
    let mut _tmp: String = String::from(value);

    _tmp.replace_range(..pos, "");
    let mut _str: &str = _tmp.trim_start();
    let _space_pos: Option<usize> = _str.find(" ");

    fill_string.clear();
    fill_string.insert_str(0, _str);
    if _space_pos != None {
        fill_string.replace_range(_space_pos.unwrap().., "");
    }
}   

/* Find starting position from flags */
fn  parse_flags(value: &String, flag: &str) -> (bool, usize) {
    let _first_pos: Option<usize> = value.rfind(flag);

    if _first_pos != None {
        let mut _str: &str = &value[_first_pos.unwrap()..];

        return (true, _first_pos.unwrap());
    }
    return (false, 0);
}

/*
 * Get position and which flags are used from a list a flags
 * then register the asked value
 */
fn  find_flags(value: &String, _max_depth: &mut String, _path: &mut String) {
    let (_rl_bool, _rl_pos) = parse_flags(value, &"-rl");
    let (_rp_bool, _rp_pos) = parse_flags(value, &"-rp");
    let (_l_bool, _l_pos) = parse_flags(value, &"-l");
    let (_p_bool, _p_pos) = parse_flags(value, &"-p");

    //println!("{_rl_bool}, {_rp_bool}, {_l_bool}, {_p_bool}");
    if _rl_bool || _l_bool {
        if _l_pos <= _rl_pos && _rl_bool {
            parse_value(value, _rl_pos + 3, _max_depth);
        } else {
            parse_value(value, _l_pos + 2, _max_depth);
        }
    }
    if _rp_bool || _p_bool {
        if _p_pos <= _rp_pos && _rp_bool {
            parse_value(value, _rp_pos + 3, _path);
        } else {
            parse_value(value, _p_pos + 2, _path);
        }
    }
}

fn main() {
    let args = std::env::args().skip(2);
    let mut _max_depth = String::from("5");
    let mut _path: String = String::from("./data/");
    let mut _url: String = String::from("");

    
    for i in args {
        if i.find('-') == Some(0) {
            find_flags(&i, &mut _max_depth, &mut _path);
        } else {
            _url = String::from(i);
        }
    }
    println!("_max_depth {_max_depth} _path {_path}");
    if _url != "" {
        println!("url: {}", _url);
    }
}
