use std::io::prelude::*;
use std::fs::{create_dir_all, File};
use tokio;
use reqwest::{ header::USER_AGENT, Client, Error, Response };

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
    if _space_pos != None {
        _fill_string.replace_range(_space_pos.unwrap().., "");
    }
}   

/* Find starting position from flags */
fn  parse_flags(value: &String, flag: &str) -> (bool, usize) {
    let first_pos: Option<usize> = value.rfind(flag);

    if first_pos != None {
        let mut _str: &str = &value[first_pos.unwrap()..];

        return (true, first_pos.unwrap());
    }
    return (false, 0);
}

/*
 * Get position and which flags are used from a list a flags
 * then register the asked value
 */
fn  find_flags(_value: &String, mut _is_recursive: bool,
    _max_depth: &mut String, _path: &mut String) {
    let (_rl_bool, _rl_pos) = parse_flags(_value, &"-rl");
    let (_rp_bool, _rp_pos) = parse_flags(_value, &"-rp");
    let (_l_bool, _l_pos) = parse_flags(_value, &"-l");
    let (_p_bool, _p_pos) = parse_flags(_value, &"-p");

    //println!("{_rl_bool}, {_rp_bool}, {_l_bool}, {_p_bool}");
    if _rl_bool || _l_bool {
        if _l_pos <= _rl_pos && _rl_bool {
            _is_recursive = true;
            parse_value(_value, _rl_pos + 3, _max_depth);
        } else {
            parse_value(_value, _l_pos + 2, _max_depth);
        }
    }
    if _rp_bool || _p_bool {
        if _p_pos <= _rp_pos && _rp_bool {
            parse_value(_value, _rp_pos + 3, _path);
        } else {
            parse_value(_value, _p_pos + 2, _path);
        }
    }
}

async fn send_url_file(_client: &Result<Client, Error>, _path: &String) -> Result<(), Error> {
    let res = _client.as_ref().unwrap()
        .get(_path.as_str())
        .header(USER_AGENT, "Reqwest/0.12.8")
        .send().await;

    match res {
        Ok(i) => {
            let mut full_path: String = ".".to_owned() + i.url().path();
            let path_clone = full_path.clone();
            eprintln!("{full_path:?}");
            let idx = full_path.rfind("/");
            match idx {
                Some(id) => {
                    println!("{id}");
                    if 1 < id {
                        let mut file_name = full_path.split_off(id);
                        file_name.insert(0, '.');
                        println!("{file_name}");
                        create_dir_all(full_path);
                        let buffer = File::create(path_clone);
                        if buffer.is_ok() {
                            let test = buffer.unwrap().write_all(&i.bytes().await?);
                            match test {
                                Ok(i) => {
                                    println!("written: {i:?}");
                                    ()
                                },
                                Err(e) => {
                                    eprint!("Couldn't write to file : {e}");
                                    ()
                                },
                            }
                        } else {
                            eprintln!("not okay");
                        }
                    } else {

                    }
                },
                None => {
                    eprintln!("Url Path should have at least a slash.");
                },
            }
            //let folder = full_path.split_off(idx.unwrap());
            //create_dir_all(full_path.clone());
            ()
        },
        Err(e) => {
            eprintln!("Error: {e:?}");
            ()
        }
    };
    //dbg!(res);
    //println!("{res:?}");
    /*
    let res_bytes = res?.bytes().await;
    if res_bytes .is_err() {
        
    }
    if res_bytes.is_ok() {
    //    dbg!(&res_bytes);
        let buffer = File::create("foo.png");
        if buffer.is_ok() {
            let aa = buffer.unwrap().write_all(&res_bytes.unwrap());
            //couldn't write
        }
    }*/
    Ok(())
    //
    //
}

async fn send_url(_client: &Result<Client, Error>, _path: &String) -> Result<String, Error> {
    let res: Result<String, Error> = _client.as_ref().unwrap().get(_path.as_str()).header(USER_AGENT, "Reqwest/0.12.8").send().await?.text().await;
    //println!("{res:?}");
    return res
}

async fn connect_client(_path: &String) -> Result<Client, Error> {
    let client: Result<Client, Error> = reqwest::Client::builder().build();

    return client
}

fn parse_request(res: &String) {
    let arr: [String; 5] = [
        ".jpg".to_string(), ".jpeg".to_string(),
        ".png".to_string(), ".gif".to_string(),
        ".bmp".to_string()
    ];
    
}

async fn send_request(_path: &String) {
    let arr: [String; 5] = [
        ".jpg".to_string(), ".jpeg".to_string(),
        ".png".to_string(), ".gif".to_string(),
        ".bmp".to_string()
    ];
    /*if !_path.starts_with("https://") {
        _path.insert_str(0, "https://");
    }*/
    let client: Result<Client, Error> = connect_client(&_path).await;
    //if link not part of arr
    let res: Result<String, Error> = send_url(&client, &_path).await;

    if res.is_ok() {
        let res_unwrap: String = res.unwrap();
    //    println!("{res_unwrap:?}");
        parse_request(&res_unwrap);
        send_url_file(&client, _path).await;
    }
}

#[tokio::main]
async fn main() {
    let args: std::iter::Skip<std::env::Args> = std::env::args().skip(2);
    let mut max_depth = String::from("5");
    let mut path: String = String::from("./data/");
    let mut url: String = String::from("");
    let mut is_recursive: bool = false;
    let mut concatenate_flag: String = String::from("");

    for i in args {
        if i.starts_with("-r") {
            is_recursive = true;
        }
        if i == "-rl" || i == "-rp" || i == "-p" || i == "-l" {
            concatenate_flag = i;
        } else {
            if i.find('-') == Some(0) || concatenate_flag != "" {
                concatenate_flag += i.as_str();
                find_flags(&concatenate_flag, is_recursive,
                    &mut max_depth, &mut path);
                concatenate_flag = String::from("");
            } else {
                url = String::from(i);
            }
        }
    }
    println!("_r: {is_recursive} max_depth {max_depth} path {path}");
    if url != "" {
        println!("url: {}", url);
        send_request(&mut url).await;

    } else {
        eprintln!("An url is needed.");
    }
}
