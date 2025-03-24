use std::collections::HashMap ;
use reqwest::{ Client, Error };
use regex::Regex;
use crate::parse_flags::parse;
mod crawl;
mod parse_document;
mod image;

async fn connect_client() -> Result<Client, Error> {
    let client: Result<Client, Error> = reqwest::Client::builder().build();

    return client
}

fn display_hmap(hmap_url: &HashMap<String, bool>) {
    println!("List url:");
    for (k, _v) in hmap_url{
        println!("{}", k);
    }
}

fn get_name_website(url: &String) -> String {
    let res_regex = Regex::new(r"^(?:https?:\/\/)([\w\d.-]*)");

    match res_regex {
        Ok(reg) => {
            let res_capture = reg.captures(url);

            if let Some(capture) = res_capture {
                let res_group = capture.get(1);

                if let Some(group) = res_group {
                    return String::from(group.as_str());
                } else {
                    eprintln!("Couldn't capture website name");
                    return "".to_owned();
                }
            } else {
                return "".to_owned(); 
            }
        },
        Err(_) => {
            eprintln!("Couldn't get website name.");
            return "".to_owned();
        }
    }
}

pub async fn connect(url: &String, path: &String, max_depth: i32) {
    let website_name = get_name_website(url);

    if website_name.is_empty() {
        return ;
    }
    let mut hmap_url: HashMap<String, bool> = HashMap::new();
    let client: &Result<Client, Error> = &connect_client().await;
    let options: parse::OptionUser = parse::OptionUser {
        url: url.clone(),
        folder: path.clone(),
        website_name: website_name.to_owned()
    };
    hmap_url.insert(options.url.clone(), false);
    match client {
        Ok(c) => {
            let mut _find_input: bool = false;

            for _i in 0..max_depth + 1{
                println!("-------------- Depth step: {_i} --------------");
                _find_input = crawl::url_helper(&options, &c, &mut hmap_url).await;
                if !_find_input {
                    break ;
                }
            }
            display_hmap(&hmap_url);
        },
        Err(e) => {
            eprintln!("Creation client Error: {e}");
        },
    }
}