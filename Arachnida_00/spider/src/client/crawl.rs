use std::collections::HashMap ;
use reqwest::{ Client, Error, Response };
use regex::Regex;
use crate::parse_flags;
use crate::client::parse_document::document;
use crate::client::image::img;

pub fn  try_insert_hmap(regex: &Result<Regex, regex::Error>,
                        hmap_url: &mut HashMap<String, bool>, f: &String, is_img: bool) {
    let k: Option<(&String, &bool)> = hmap_url.get_key_value(f);

    match &regex {
        Ok(reg) => {
            let res_captures: Option<regex::Captures<'_>> = reg.captures(&f);

            if let Some(capture) = res_captures {
                let res_url = capture.get(0);

                if let Some(url) = res_url {
                    if !url.is_empty() {
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
                    } else {
                        eprintln!("Url: {f}");
                        eprintln!("Url not accepted, must be of same website crawled.");
                    }
                }
            } else {
                eprintln!("Couldn't capture url");
            }
        },
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }
}

/*
 * Call server (request) to parse
 */
async fn get_url_header(client: &Client, path: &str) -> Result<Response, Error> {
    let res: Result<Response, Error> = client.get(path).send().await;

    res
}


pub async fn url_helper(options: &parse_flags::parse::OptionUser, cli: &Client,
    hmap_url: &mut HashMap<String, bool>) -> bool {
    let mut find_input: bool = false;

    for (k, v) in hmap_url.clone() {
        if v == true {
            continue;
        }
        println!("Url open: {k}");
        hmap_url.insert(k.to_owned(), true);
        let res: Result<Response, Error> = get_url_header(cli, &k).await;

        match res {
            Ok(r) => {
                let content_type: Option<&reqwest::header::HeaderValue>
                            = r.headers().get("content-type");
                let res: bool = img::check_image_type(&content_type);

                if !res {
                    let res_txt: Result<String, Error> = r.text().await;

                    match res_txt {
                        Ok(txt) => {
                            find_input = document::parse_doc(&options, cli, &txt, hmap_url).await;
                        },
                        Err(e) => {
                            eprintln!("Get Url Error: {e}");
                        },
                    }
                } else {
                    let _ = img::process_image(r, options).await;
                }
            },
            Err(e) => {
                eprintln!("Error: {e}");
            },
        }
    }
    find_input
}