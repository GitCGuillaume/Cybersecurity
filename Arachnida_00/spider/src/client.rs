use futures::{
    prelude::Future,
    future::FutureExt
};
use std::collections::HashMap;
use reqwest::{ Client, Error };
use std::boxed::Box;
use core::pin::Pin;
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
                    step: i32, max_depth: i32) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    async move {
        if max_depth <= step {
            return ;
        }
        println!("-------------- Depth Step: {0} --------------", step + 1);
        let _find_input: bool = crawl::url_helper(&options, &cli, hmap_url).await;

        if !_find_input {
            return ;
        }
        recursive_download(&options, cli, hmap_url, step + 1, max_depth).await;
    }.boxed_local()
}

pub async fn connect(options: &parse::OptionUser, max_depth: i32) {

    let mut hmap_url: HashMap<String, bool> = HashMap::new();
    let client: &Result<Client, Error> = &connect_client().await;

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