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

fn launch_connection(options: &mut parse::OptionUser) {
    if !options.is_recursive {
        if options.max_depth != 5 {
            println!("Recusivity is not activated, set maximum depth to 0 by default.");
        }
        options.max_depth = 0;
    }
    let website_name: String = get_name_website(&options.url);

    if website_name.is_empty() {
        return ;
    }
    options.website_name = website_name.to_owned();
    client::connect(&options, options.max_depth);
}

/*
 * List flags :
 *  -rl recursive with depth
 *  -rp recursive with image path to register
 *  -l depth recursivity, default max depth is 5
 *  -p choose image path to register
 * https://github.com/tokio-rs/tokio/issues/4756
 */
fn main() {
    let args: std::iter::Skip<std::env::Args> = std::env::args().skip(1);
    let mut options: parse::OptionUser = parse::OptionUser {
        url: String::from(""),
        folder: String::from("./data/"),
        website_name: String::from(""),
        max_depth: 5,
        is_recursive: false
    };
    let mut concat: String = String::from("");

    for i in args {
        concat.push_str(i.as_str());
        concat.push(' ');
    }
    concat = parse_flags::parse::get_url(&mut concat, &mut options);
    if concat.is_empty() {
        eprintln!("Please provide a URL with of format of (http(s)://name)");
        return ;
    }
    if parse_flags::parse::find_flags(&concat, &mut options) == false {
        return ;
    }
    launch_connection(&mut options);
}
