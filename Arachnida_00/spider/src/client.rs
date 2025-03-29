use std::collections::HashMap;
use crate::parse_flags::parse;
mod crawl;
mod parse_document;
mod image;
use reqwest::blocking::Client;
use reqwest::Error;

fn connect_client() -> Result<Client, Error> {
    let client: Result<Client, Error> = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .user_agent("Reqwest/0.12.8").build();
    return client
}

fn display_hmap(hmap_url: &HashMap<String, bool>) {
    println!("List url:");
    for (k, _v) in hmap_url {
        println!("{}", k);
    }
}

/*
 * Recursively download
 * School rustc version is 1.7.5, can't use Box::pin
 * https://docs.rs/futures/0.3.31/futures/prelude/trait.Future.html
 */
fn recursive_download<'a>(options: &'a parse::OptionUser, cli: &'a Client,
                    hmap_url: &'a mut HashMap<String, bool>,
                    step: i32, max_depth: i32) {
    if max_depth <= step {
        return ;
    }
    println!("-------------- Depth Step: {0} --------------", step + 1);
    let _find_input: bool = crawl::url_helper(&options, &cli, hmap_url);

    if !_find_input {
        return ;
    }
    recursive_download(&options, cli, hmap_url, step + 1, max_depth);
}

pub fn connect(options: &parse::OptionUser, max_depth: i32) {

    let mut hmap_url: HashMap<String, bool> = HashMap::new();
    let client: Result<Client, Error> = connect_client();

    hmap_url.insert(options.url.clone(), false);
    match client {
        Ok(cli) => {
            println!("-------------- Initial step --------------");
            let _find_input: bool = crawl::url_helper(&options, &cli, &mut hmap_url);

            if !_find_input {
                return ;
            }
            recursive_download(&options, &cli, &mut hmap_url, 0, max_depth);
            display_hmap(&hmap_url);
        },
        Err(e) => {
            eprintln!("Creation client Error: {e}");
        },
    }
}
