pub mod img {
    use reqwest::{ header::USER_AGENT, Client, Error, Response };
    use std::collections::HashMap ;
    use std::fs::{ create_dir_all, File };
    use select::document::{ Document, Find };
    use select::predicate::Name;
    use regex::Regex;
    use std::io::prelude::Write;
    use crate::parse_flags::parse;
    use crate::client::crawl;

    /*
    * https://www.iana.org/assignments/media-types/media-types.xhtml#image
    * Check if content-type is valid
    */
    pub fn check_image_type(content_type: &Option<&reqwest::header::HeaderValue>) -> bool {
        let arr_type: [String; 5] = [
            "image/jpg".to_string(), "image/jpeg".to_string(),
            "image/png".to_string(), "image/gif".to_string(),
            "image/bmp".to_string()
        ];
        let result: bool = match content_type {
            Some(content) => {
                let name: Result<&str, reqwest::header::ToStrError> = content.to_str();
                let mut is_same = false;

                match name {
                    Ok(value) => {
                        for i in arr_type {
                            if i == value {
                                is_same = true;
                            }
                        }
                        ()
                    },
                    Err(_) => {
                        ()
                    },
                };
                is_same
            },
            None => {
                return false;
            },
        };
        result
    }

    /*
        Create directory and it's childs
    */
    fn create_dir(_dir_path: &mut String, id: usize) {
        let _file_name: String = _dir_path.split_off(id);
        println!("{_file_name}");
        let result_dir: Result<(), std::io::Error> = create_dir_all(_dir_path);

        match result_dir {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Something went wrong wih directories creation: {e}");
                ()
            },
        }
    }

    /*
    * Create and write in file
    */
    fn  create_image(_img_path: &String, i: &[u8]) {
        let buffer: Result<File, std::io::Error> = File::create_new(_img_path);

        match buffer {
            Ok(mut f) => {
                let res: Result<(), std::io::Error> = f.write_all(i);
                match res {
                    Ok(_) => {
                        println!("Image created")
                    },
                    Err(e) => {
                        eprintln!("Couldn't write to file: {e}");
                    },
                }
            },
            Err(e) => {
                eprintln!("Error: {e}");
            },
        }
    }

    /* Start of image creation, call all functions to create an image */
    pub async fn process_image(i: Response, options: &parse::OptionUser) -> Result<(), Error> {
        let content_type: Option<&reqwest::header::HeaderValue>
                                = i.headers().get("content-type");
        let res: bool = check_image_type(&content_type);

        if res == false {
            eprintln!("File is not part of accepted type-content");
            return Ok(());
        }
        let mut relative_path: String = options.folder.clone() + i.url().path();
        let path_clone = relative_path.clone();
        let idx: Option<usize> = relative_path.rfind("/");

        match idx {
            Some(id) => {
                if 1 < id {
                    create_dir(&mut relative_path, id);
                    create_image(&path_clone, &i.bytes().await?);
                } else {
                    create_image(&path_clone, &i.bytes().await?);
                }
            },
            None => {
                eprintln!("Url Path should have at least a slash.");
            },
        }
        Ok(())
    } 

    /*
    * Function wrapping image creation and validation
    */
    async fn send_url_file(_client: &Client, options: &parse::OptionUser, _path: &str,) -> Result<(), Error> {
        let res: Result<Response, Error> = _client.get(_path)
            .header(USER_AGENT, "Reqwest/0.12.8")
            .send().await;

        let res: Result<(), Error> = match res {
            Ok(i) => {
                let _ = process_image(i, options).await;

                Ok(())
            },
            Err(e) => {
                eprintln!("Error: {e:?}");
                Err(e)
            }
        };
        res
    }

    pub async fn  get_images(options: &parse::OptionUser, cli: &Client,
                        _doc: &Document, hmap_url: &mut HashMap<String, bool>) {
        let regex = Regex::new(r"^https?://[\w\d.-]*");
        let img_dom: Find<'_, Name<&str>> = _doc.find(Name("img"));

        for f in img_dom.filter_map(|f| f.attr("src")) {
            if !f.starts_with("http") {
                match &regex {
                    Ok(reg) => {
                        let captures = reg.captures(&options.url).unwrap().get(0).unwrap().as_str();
                        let new_url: String;

                        if !f.starts_with("/") {
                            new_url = String::from(captures) + "/" + f;
                            println!("{new_url}");
                        } else {
                            new_url = String::from(captures) + f;
                            println!("{new_url}");
                        }

                        crawl::try_insert_hmap(hmap_url, &new_url, true);
                        let _ = send_url_file(cli, options, &new_url).await;
                    },
                    Err(_) => {},
                };
            } else {
                    crawl::try_insert_hmap(hmap_url, &f.to_owned(), true);
                    let _ = send_url_file(cli, options, &f).await;
            }
        }
    }
}