use std::collections::HashMap ;
use reqwest::{ Client, Error };
use crate::parse_flags::parse;
mod crawl;
mod parse_document;
mod image;

async fn connect_client(_path: &String) -> Result<Client, Error> {
    let client: Result<Client, Error> = reqwest::Client::builder().build();

    return client
}

pub async fn connect(_url: &String, path: &String, _max_depth: i32) {
    let mut hmap_url: HashMap<String, bool> = HashMap::new();
    let client: &Result<Client, Error> = &connect_client(&_url).await;
    let options: parse::OptionUser = parse::OptionUser {
        url: _url.clone(),
        folder: path.clone()
    };

    hmap_url.insert(_url.clone(), false);
    match client {
        Ok(c) => {
            let mut _find_input: bool = false;

            for _i in 0.._max_depth {
                println!("Depth: {_i}");
                _find_input = crawl::url_helper(&options, &c, &mut hmap_url).await;
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