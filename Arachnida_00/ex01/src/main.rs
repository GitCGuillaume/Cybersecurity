use std::collections::HashMap ;
use std::io::prelude::*;
use std::fs::{ create_dir_all, File };
use regex::Regex;
use select::document::Document;
use select::predicate::Name;
use tokio;
use reqwest::{ header::USER_AGENT, Client, Error, Response };
use select::document;

struct OptionUser {
    url: String,
    folder: String
}

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
fn check_image_type(content_type: &Option<&reqwest::header::HeaderValue>) -> bool {
    let arr_type: [String; 5] = [
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
                    //println!("Type-content: {value}");
                    for i in arr_type {
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
    println!("d: {_dir_path}");
    let _file_name: String = _dir_path.split_off(id);
    println!("dspl: {_dir_path}");
    println!("{_file_name}");
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
                Ok(_) => {
                    println!("Image created")
                },
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
async fn process_image(i: Response, options: &OptionUser) -> Result<(), Error> {
    //println!("{i:?}");
    let content_type: Option<&reqwest::header::HeaderValue>
                            = i.headers().get("content-type");
    let res: bool = check_image_type(&content_type);

    if res == false {
        //eprintln!("File is not part of accepted type-content");
        return Ok(());
    }
    let mut relative_path: String = options.folder.clone() + i.url().path();
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
async fn send_url_file(_client: &Client, options: &OptionUser, _path: &str,) -> Result<(), Error> {
    let res: Result<Response, Error> = _client.get(_path)
        .header(USER_AGENT, "Reqwest/0.12.8")
        .send().await;

    let res: Result<(), Error> = match res {
        Ok(i) => {
            let _ = process_image(i, options).await;

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
async fn get_url_header(_client: &Client, _path: &str) -> Result<Response, Error> {
    let res: Result<Response, Error> = _client.get(_path).header(USER_AGENT, "Reqwest/0.12.8").send().await;

    res
}

fn  try_insert_hmap(hmap_url: &mut HashMap<String, bool>, f: &String, is_img: bool) {
    let k: Option<(&String, &bool)> = hmap_url.get_key_value(f);

    match k {
        Some(_) => (),
        None => {
            if is_img {
                hmap_url.insert(f.to_owned(), true);
            } else {
                hmap_url.insert(f.to_owned(), false);
            }
        },
    }
}

fn  get_links(_url: &String, _doc: &Document,
             hmap_url: &mut HashMap<String, bool>) -> bool {
    let regex = Regex::new(r"^https?://[\w\d.-]*");
    let a_dom: document::Find<'_, Name<&str>> = _doc.find(Name("a"));
    let mut find_input: bool = false;

    a_dom.filter_map(|f| f.attr("href"))
        .for_each(|f| {
            find_input = true;
            if !f.starts_with("http") {
                match &regex {
                    Ok(reg) => {
                        let aa = reg.captures(_url).unwrap().get(0).unwrap().as_str();
                        let bb = String::from(aa) + f;

                        try_insert_hmap(hmap_url, &bb, false);
                    },
                    Err(_) => {},
                };
            } else {
                try_insert_hmap(hmap_url, &f.to_owned(), false);

            }
        });
    find_input
}

async fn  get_images(options: &OptionUser, cli: &Client,
                     _doc: &Document, hmap_url: &mut HashMap<String, bool>) {
    let regex = Regex::new(r"^https?://[\w\d.-]*");
    let img_dom: document::Find<'_, Name<&str>> = _doc.find(Name("img"));

    for f in img_dom.filter_map(|f| f.attr("src")) {
        if !f.starts_with("http") {
            match &regex {
                Ok(reg) => {
                    let aa = reg.captures(&options.url).unwrap().get(0).unwrap().as_str();
                    let bb = String::from(aa) + f;

                    try_insert_hmap(hmap_url, &bb, true);
                    let _ = send_url_file(cli, options, &bb).await;
                },
                Err(_) => {},
            };
        } else {
                try_insert_hmap(hmap_url, &f.to_owned(), true);
                let _ = send_url_file(cli, options, &f).await;
        }
    }
}

async fn parse_document(options: &OptionUser, cli: &Client,
                 text: &String, hmap_url: &mut HashMap<String, bool>) -> bool {
    let doc: Document = document::Document::from(text.as_str());
    let mut _find_input: bool = false;

    _find_input = get_links(&options.url, &doc, hmap_url);
    get_images(options, cli, &doc, hmap_url).await;
    _find_input
}

async fn url_helper(options: &OptionUser, cli: &Client,
    hmap_url: &mut HashMap<String, bool>) -> bool {
    let mut find_input: bool = false;

    for (k, _v) in hmap_url.clone() {
        if _v == true {
            continue;
        }
        println!("Url open: {k}");
        hmap_url.insert(k.to_owned(), true);
        let res: Result<Response, Error> = get_url_header(cli, &k).await;

        match res {
            Ok(r) => {
                let content_type: Option<&reqwest::header::HeaderValue>
                            = r.headers().get("content-type");
                let res: bool = check_image_type(&content_type);
                if !res {
                    let txt: Result<String, Error> = r.text().await;

                    match txt {
                        Ok(r) => {
                            find_input = parse_document(&options, cli, &r, hmap_url).await;
                        },
                        Err(e) => {
                            eprintln!("Get Url Error: {e}");
                        },
                    }
                } else {
                    let _ = process_image(r, options).await;
                }
            },
            Err(e) => {
                eprintln!("Error: {e}");
            },
        }
    }
    find_input
}

async fn connect_client(_path: &String) -> Result<Client, Error> {
    let client: Result<Client, Error> = reqwest::Client::builder().build();

    return client
}

async fn connect(_url: &String, path: &String, _max_depth: i32) {
    let mut hmap_url: HashMap<String, bool> = HashMap::new();
    let client: &Result<Client, Error> = &connect_client(&_url).await;
    let options: OptionUser = OptionUser {
        url: _url.clone(),
        folder: path.clone()
    };

    hmap_url.insert(_url.clone(), false);
    match client {
        Ok(c) => {
            let mut _find_input: bool = false;

            for _i in 0.._max_depth {
                println!("i: {_i}");
                _find_input = url_helper(&options, &c, &mut hmap_url).await;
                if !_find_input {
                    break ;
                }
            }
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
        //println!("url: {}", url);
        let max_depth: Result<i32, _> = max_depth.parse();
        match max_depth {
            Ok(max) => {
                connect(&url, &path, max).await;
            },
            Err(_) => {
                eprint!("Please provide a valid max depth.");
            },
        }
    } else {
        eprintln!("An url is needed.");
    }
}
