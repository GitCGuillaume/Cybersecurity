use std::collections::HashMap ;
use reqwest::{ Client, Error };
use regex::Regex;
use std::boxed::Box;
use crate::parse_flags::parse;
mod crawl;
mod parse_document;
mod image;

async fn connect_client() -> Result<Client, Error> {
    let client: Result<Client, Error> = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .user_agent("Reqwest/0.12.8").build();

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
                eprintln!("Coudln't parse URL: {}", url);
                return "".to_owned(); 
            }
        },
        Err(_) => {
            eprintln!("Couldn't get website name.");
            return "".to_owned();
        }
    }
}

async fn recursive_download(options: &parse::OptionUser, cli: &Client,
                    hmap_url: &mut HashMap<String, bool>,
                    step: i32, max_depth: i32)  {
    if max_depth <= step {
        return ;
    }
    println!("-------------- Depth Step: {0} --------------", step + 1);
    let _find_input: bool = crawl::url_helper(&options, &cli, hmap_url).await;
    if !_find_input {
        return ;
    }
    Box::pin(recursive_download(&options, cli, hmap_url, step + 1, max_depth)).await;
}

pub async fn connect(url: &String, path: &String, max_depth: i32) {
    let website_name: String = get_name_website(url);

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
        Ok(cli) => {
            println!("-------------- Initial step --------------");
            let _find_input: bool = crawl::url_helper(&options, &cli, &mut hmap_url).await;

            if !_find_input {
                return ;
            }
            recursive_download(&options, &cli, &mut hmap_url, 0, max_depth).await;
            display_hmap(&hmap_url);
        },
        Err(e) => {
            eprintln!("Creation client Error: {e}");
        },
    }
}