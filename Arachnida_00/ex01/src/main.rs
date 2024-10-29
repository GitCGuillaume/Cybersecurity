use std::io::prelude::*;
use std::fs::{create_dir_all, File};
use select::document::Document;
use select::predicate::{Any, Element, Text, Name};
use tokio;
use reqwest::{ header::USER_AGENT, Client, Error, Response };
use select::{ document};

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
fn  find_flags(_value: &String, mut _is_recursive: bool,
    _max_depth: &mut String, _path: &mut String) {
    let (_rl_bool, _rl_pos) = parse_flags(_value, &"-rl");
    let (_rp_bool, _rp_pos) = parse_flags(_value, &"-rp");
    let (_l_bool, _l_pos) = parse_flags(_value, &"-l");
    let (_p_bool, _p_pos) = parse_flags(_value, &"-p");

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

/*
 * https://www.iana.org/assignments/media-types/media-types.xhtml#image
 * Check if content-type is valid
 */
fn check_image_type(content_type: Option<&reqwest::header::HeaderValue>) -> bool {
    let arr: [String; 5] = [
        "image/jpg".to_string(), "image/jpeg".to_string(),
        "image/png".to_string(), "image/gif".to_string(),
        "image/bmp".to_string()
    ];
    let result: bool = match content_type {
        Some(content) => {
            let name: Result<&str, reqwest::header::ToStrError> = content.to_str();
            let mut is_same = false;
            match name {
                Ok(value) => {
                    for i in arr {
                        println!("{i} {value}");
                        if i == value {
                            is_same = true;
                        }
                    }
                    ()
                },
                Err(_) => {
                    ()
                },
            };
            is_same
        },
        None => {
            return false;
        },
    };
    result
}

/*
    Create directory and it's childs
*/
fn create_dir(_dir_path: &mut String, id: usize) {
    let _file_name: String = _dir_path.split_off(id);
    let result_dir: Result<(), std::io::Error> = create_dir_all(_dir_path);

    match result_dir {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Something went wrong wih directories creation: {e}");
            ()
        },
    }
}

/*
 * Create and write in file
 */
fn  create_image(_img_path: &String, i: &[u8]) {
    let buffer: Result<File, std::io::Error> = File::create_new(_img_path);

    match buffer {
        Ok(mut f) => {
            let res: Result<(), std::io::Error> = f.write_all(i);
            match res {
                Ok(result) => println!("{result:?}"),
                Err(e) => {
                    eprintln!("Couldn't write to file: {e}");
                },
            }
        },
        Err(e) => {
            eprintln!("Error: {e}");
        },
    }
}

/* Start of image creation, call all functions to create an image */
async fn process_image(i: Response) -> Result<(), Error> {
    let content_type: Option<&reqwest::header::HeaderValue>
                            = i.headers().get("content-type");
    let res: bool = check_image_type(content_type);

    if res == false {
        eprintln!("File is not part of accepted type-content");
        return Ok(());
    }
    let mut relative_path: String = ".".to_owned() + i.url().path();
    let path_clone = relative_path.clone();
    let idx: Option<usize> = relative_path.rfind("/");

    match idx {
        Some(id) => {
            if 1 < id {
                create_dir(&mut relative_path, id);
                create_image(&path_clone, &i.bytes().await?);
            } else {
                create_image(&path_clone, &i.bytes().await?);
            }
        },
        None => {
            eprintln!("Url Path should have at least a slash.");
        },
    }
    Ok(())
} 

/*
 * Function wrapping image creation and validation
 */
async fn send_url_file(_client: &Client, _path: &str) -> Result<(), Error> {
    let res: Result<Response, Error> = _client.get(_path)
        .header(USER_AGENT, "Reqwest/0.12.8")
        .send().await;

    let res: Result<(), Error> = match res {
        Ok(i) => {
            let _ = process_image(i).await;
            Ok(())
        },
        Err(e) => {
            eprintln!("Error: {e:?}");
            Err(e)
        }
    };
    res
}

/*
 * Call server (request) to parse
 */
async fn get_url_text(_client: &Client, _path: &str) -> Result<String, Error> {
    let res: Result<String, Error> = match _client.get(_path).header(USER_AGENT, "Reqwest/0.12.8").send().await {
        Ok(value) => {
            value.text().await
        },
        Err(e) => {
            Err(e)
        },
    };
    res
}

async fn connect_client(_path: &String) -> Result<Client, Error> {
    let client: Result<Client, Error> = reqwest::Client::builder().build();

    return client
}

/*
 * Resolve array and return a boolean
 */
fn  url_extension_resolver(_path: &str, arr: &[String; 5]) -> bool {
    for i in arr {
        if _path.ends_with(i) {
            return true;
        }
    }
    false
}

fn  get_links(cli:&Client, arr: &[String; 5],
    _doc: &Document, v_fill: &mut Vec<String>) -> bool {
    let img: document::Find<'_, Name<&'static str>> = _doc.find(Name("a"));
    let mut find_input = false;

    img.filter_map(|f| f.attr("href"))
        .for_each(|f| {
            //println!("{f}");
            find_input = true;
            v_fill.push(f.to_owned());
            //url_helper(cli, arr, &"".to_string(), _max_depth);
        });
    find_input
}

fn parse_document(cli: &Client, text: &String,
    arr: &[String; 5], v_fill: &mut Vec<String>) -> bool {
    let doc: Document = document::Document::from(text.as_str());

    return get_links(cli, arr, &doc, v_fill);
}

async fn url_helper(cli: &Client, arr: &[String; 5],
    v_list: &Vec<String>, v_fill: &mut Vec<String>) -> bool {
    
    for i in v_list {
        if url_extension_resolver(i, &arr) {
            //println!("true");
            let _ = send_url_file(cli, i).await;
        } else {
            //println!("false");
            let res: Result<String, Error> = get_url_text(cli, &i).await;
    
            match res {
                Ok(r) => {
                    return parse_document(cli, &r, arr, v_fill);
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                },
            }
        }
    }
    false
}

async fn connect(_path: &String, _max_depth: i32) {
    let arr: [String; 5] = [
        ".jpg".to_string(), ".jpeg".to_string(),
        ".png".to_string(), ".gif".to_string(),
        ".bmp".to_string()
    ];
    /*if !_path.starts_with("https://") {
        _path.insert_str(0, "https://");
    }*/
    let client: &Result<Client, Error> = &connect_client(&_path).await;

    match client {
        Ok(c) => {
            let mut v1: Vec<String> = Vec::new();
            let mut v2: Vec<String> = Vec::new();

            v1.reserve(1024);
            v1.reserve(1024);
            v1.push(_path.to_owned());
            //loop max_depth
            for i in 0.._max_depth {
                //choose vector to loop through, and vector to fill
                if v1.len() == 0 {
                    //v2, v1 gogo
                    url_helper(&c, &arr, &v2, &mut v1).await;
                } else {
                    //v1, v2 gogo
                    url_helper(&c, &arr, &v1, &mut v2).await;
                }
            }
            dbg!(v2);
            println!("{_max_depth}");
            //println!("{i}");
        },
        Err(e) => {
            eprintln!("Creation client Error: {e}");
        },
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
        let max_depth: Result<i32, _> = max_depth.parse();
        match max_depth {
            Ok(max) => {
                connect(&url, max).await;
            },
            Err(_) => {
                eprint!("Please provide a valid max depth.");
            },
        }
    } else {
        eprintln!("An url is needed.");
    }
}
