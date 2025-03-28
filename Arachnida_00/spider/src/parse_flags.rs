/* Find starting position from flags */
fn  parse_flags(value: &String, flag: &str) -> (bool, isize) {
    let first_pos: Option<usize> = value.rfind(flag);
    
    match first_pos {
        Some(pos) => {
            let mut _str: &str = &value[pos..];

            return (true, pos.try_into().unwrap());
        },
        None => return (false, -1)
    };
}

pub mod parse {
    use crate::parse_flags;
    pub struct OptionUser {
        pub url: String,
        pub folder: String,
        pub website_name: String,
        pub max_depth: i32,
        pub is_recursive: bool
    }

    /*
     * Check max depth, return max depth to default if a flag
     * then check if max depth is a signed integer 32
     */
    fn convert_string_to_integer(cmd: &str, max_depth: &str, options: &mut OptionUser) -> bool {
        let res_max: Result<i32, _> = max_depth.parse();

        match res_max {
            Ok(val) => {
                if val < 0 {
                    eprintln!("Please provide a positive max depth for {cmd}: {val}");
                }
                options.max_depth = val;
                return true;
            },
            Err(err) => {
                eprintln!("Please provide a valid max depth for {cmd}: {err}");
                return false;
            }
        }
    }

    /* Check current asked flag, register current flag in mem_str */
    fn match_current(i: &str, next: &mut bool, mem_str: &mut String) -> bool {
        match i {
            "-rlp" => {
                *next = true;
                *mem_str = String::from(i);
            },
            "-rl" => {
                *next = true;
                *mem_str = String::from(i);
            },
            "-rp" => {
                *next = true;
                *mem_str = String::from(i);
            },
            "-l" => {
                *next = true;
                *mem_str = String::from(i);
            },
            "-p" => {
                *next = true;
                *mem_str = String::from(i);
            },
            "-r" => {},
            _ => {
                if !i.is_empty() {
                    eprintln!("Please provide correct arguments: {i}");
                    return false;
                }
            }
        }
        return true;
    }

    /*
    * check mem_str value from previous flag, return false is something went wrong
    * Otherwise true
    */
    fn match_next(i: &str, mem_str: &String, options: &mut OptionUser) -> bool {
        match mem_str.as_str() {
            "-rlp" => {
                if i.is_empty() || i == mem_str.as_str() {
                    eprintln!("Please provide a path after -rlp");
                    return false;
                }
                options.folder = String::from(i);
            },
            "-rl" => {
                if convert_string_to_integer(&mem_str, i, options) == false {
                    return false;
                }
            },
            "-rp" => {
                if i.is_empty() {
                    eprintln!("Please provide a path after -rp");
                    return false;
                }
                options.folder = String::from(i);
            },
            "-l" => {
                if convert_string_to_integer(&mem_str, i, options) == false {
                    return false;
                }
            },
            "-p" => {
                if i.is_empty() {
                    eprintln!("Please provide a path after -p");
                    return false;
                }
                options.folder = String::from(i);
            },
            _ => {}
        }
        return true;
    }

    /* Parse flags */
    pub fn find_flags(concat: &String, options: &mut OptionUser) -> bool {
        let mut mem_str: String = String::from("");
        let (r_bool, r_pos) = parse_flags::parse_flags(concat, &"-r");
        let split: Vec<_> = concat.split(" ").collect();
        let mut next = false;

        if r_bool && r_pos == 0 {
            options.is_recursive = true;
        }
        for i in split {
            if !next {
                if match_current(i, &mut next, &mut mem_str) == false {
                    return false;
                }
            } else {
                if i == "-rlp" || i == "-rl"
                    || i == "-rp" || i == "-l" || i == "-p" || i == "-r" {
                        eprintln!("Bad arguments, please provide correct arguments as");
                        eprintln!("\tspider [-r -l argument p argument] URL");
                        return false;
                }
                if match_next(&i, &mem_str, options) == false {
                    return false;
                }
                mem_str = String::from("");
                next = false;
            }
        }
        true
    }

    /*
     * Split and register url in options
     */
    pub fn get_url(concat: &mut String, options: &mut OptionUser) -> String {
        let mut find_idx = None;
        let split: Vec<_> = concat.split(" ").collect();

        for i in split {
            find_idx = i.find("http://");
            if find_idx == None {
                find_idx = i.find("https://");
                if find_idx == None {
                    continue ;
                } else {
                    options.url = String::from(i);
                    break ;
                }
            } else {
                options.url = String::from(i);
                break ;
            }
        }
        if find_idx == None {
            return String::from("");
        }
        if find_idx == Some(0) {
            return concat.replace(&options.url, "");
        }
        return String::from("");
    }
}