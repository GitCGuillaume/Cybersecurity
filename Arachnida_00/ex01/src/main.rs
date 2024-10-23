use tokio;
use reqwest::{self, Client, Error, Response};

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
fn  find_flags(value: &String, mut _is_recursive: bool,
    _max_depth: &mut String, _path: &mut String) {
    let (_rl_bool, _rl_pos) = parse_flags(value, &"-rl");
    let (_rp_bool, _rp_pos) = parse_flags(value, &"-rp");
    let (_l_bool, _l_pos) = parse_flags(value, &"-l");
    let (_p_bool, _p_pos) = parse_flags(value, &"-p");

    //println!("{_rl_bool}, {_rp_bool}, {_l_bool}, {_p_bool}");
    if _rl_bool || _l_bool {
        if _l_pos <= _rl_pos && _rl_bool {
            _is_recursive = true;
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

async fn send(_client: &Result<Client, Error>, _path: &String) -> Result<Response, Error> {
    let _res: Result<Response, Error> = _client.as_ref().unwrap().get(_path.as_str()).send().await;

    return _res
}

async fn connect_client(_path: &String) -> Result<Client, Error> {
    let _client: Result<Client, Error> = reqwest::Client::builder().build();

    return _client
}

async fn send_request(_path: &mut String) {
    /*if !_path.starts_with("https://") {
        _path.insert_str(0, "https://");
    }*/
    let _client: Result<Client, Error> = connect_client(&_path).await;
    let _res: Result<Response, Error> = send(&_client, &_path).await;

    if _res.is_ok() {
        dbg!(_res);
    }
}

#[tokio::main]
async fn main() {
    let args: std::iter::Skip<std::env::Args> = std::env::args().skip(2);
    let mut _max_depth = String::from("5");
    let mut _path: String = String::from("./data/");
    let mut _url: String = String::from("");
    let mut _is_recursive: bool = false;
    let mut _concatenate_flag: String = String::from("");

    for i in args {
        if i.starts_with("-r") {
            _is_recursive = true;
        }
        if i == "-rl" || i == "-rp" || i == "-p" || i == "-l" {
            _concatenate_flag = i;
        } else {
            if i.find('-') == Some(0) || _concatenate_flag != "" {
                _concatenate_flag += i.as_str();
                find_flags(&_concatenate_flag, _is_recursive,
                    &mut _max_depth, &mut _path);
                _concatenate_flag = String::from("");
            } else {
                _url = String::from(i);
            }
        }
    }
    println!("_r: {_is_recursive} _max_depth {_max_depth} _path {_path}");
    if _url != "" {
        println!("url: {}", _url);
        send_request(&mut _url).await;

    } else {
        eprintln!("An url is needed.");
    }
}
