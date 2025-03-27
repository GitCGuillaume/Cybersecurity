use regex::Regex;
use tokio::runtime::Runtime;
mod client;
mod parse_flags;
use crate::parse_flags::parse;

fn get_name_website(url: &String) -> String {
    let find_idx = url.find("http://");

    if find_idx == None {
        let find_idx = url.find("https://");
        
        if find_idx == None {
            eprintln!("Please provide a correct URL with correct protocol format: {url}");
            return "".to_owned();
        }
    }
    let split: Vec<_> = url.split("/").collect();
println!("split:{:?}", split);
    if split.len() < 3 {
        eprintln!("Please provide a correct URL with correct protocol format: {url}");
        return "".to_owned();
    }
    if split[2].is_empty() {
        eprintln!("Please provide a correct URL.");
        return "".to_owned();
    }
    return String::from(split[2]);
}

fn launch_connection(is_recursive: bool, max_depth: &mut String,
    url: &String, path: &String) {
    if !is_recursive {
        if max_depth != "5" {
            println!("Recusivity is not activated, set maximum depth to 1 by default.");
        }
        max_depth.clear();
        max_depth.push_str("0");
    }
    let max_depth: Result<i32, _> = max_depth.parse();
    let website_name: String = get_name_website(url);

    if website_name.is_empty() {
        return ;
    }
    let options: parse::OptionUser = parse::OptionUser {
        url: url.clone(),
        folder: path.clone(),
        website_name: website_name.to_owned()
    };

    match max_depth {
        Ok(max) => {
             let rt: Result<Runtime, std::io::Error>  = Runtime::new();

             match rt {
                Ok(r) => {
                    r.block_on(async {
                        client::connect(&options, max).await;
                    });
                },
                Err(e) => {eprintln!("Error: {e}")},
            }
        },
        Err(_) => {
            eprintln!("Please provide a valid max depth.");
        },
    }
}

/*
 * List flags :
 *  -rl recursive with depth
 *  -rp recursive with image path to register
 *  -l depth recursivity, default max depth is 5
 *  -p choose image path to register
 */
fn main() {
    let args: std::iter::Skip<std::env::Args> = std::env::args().skip(1);
    let mut max_depth: String = String::from("5");
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
                parse_flags::parse::find_flags(&concatenate_flag, is_recursive,
                    &mut max_depth, &mut path);
                concatenate_flag = String::from("");
            } else {
                url = String::from(i);
            }
        }
    }
    if url != "" {
        launch_connection(is_recursive, &mut max_depth,
            &url, &path);
    } else {
        eprintln!("An url is needed.");
    }
}
