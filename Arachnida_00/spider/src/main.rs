use tokio;
mod client;
mod parse_flags;

async fn launch_connection(is_recursive: bool, max_depth: &mut String,
    url: &String, path: &String) {
    if !is_recursive {
        if max_depth != "5" {
            println!("Recusivity is not activated, set maximum depth to 1 by default");
        }
        max_depth.clear();
        max_depth.push_str("1");
    }
    let max_depth: Result<i32, _> = max_depth.parse();

    match max_depth {
        Ok(max) => {
            client::connect(&url, &path, max).await;
        },
        Err(_) => {
            eprint!("Please provide a valid max depth.");
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
#[tokio::main]
async fn main() {
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
            &url, &path).await;
    } else {
        eprintln!("An url is needed.");
    }
}
