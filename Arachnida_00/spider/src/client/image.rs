pub mod img {
    use reqwest::{ header::USER_AGENT, Client, Error, Response };
    use std::collections::HashMap ;
    use std::fs::{ create_dir_all, File };
    use select::document::{ Document, Find };
    use select::predicate::Name;
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
        Split path Folder and image
        Create directory and it's childs
    */
    fn create_dir(dir_path: &mut String, id: usize) -> bool {
        let _file_name: String = dir_path.split_off(id);
        let result_dir: Result<(), std::io::Error> = create_dir_all(dir_path);

        return match result_dir {
            Ok(_) => {
                return true;
            },
            Err(e) => {
                eprintln!("Something went wrong wih directories creation: {e}");
                false
            },
        }
    }

    /*
     * Open file
     */
    fn open_file(img_path: &String) -> bool {
        let file = File::open(img_path);

        return match file {
            Ok(_) => {
                true
            },
            Err(_) => {
                false
            }
        };
    }

    /*
    * Create and write in file
    */
    fn  create_image(img_path: &String, i: &[u8]) {
        let f: bool = open_file(img_path);

        if f {
            eprintln!("File already exist");
        } else {
            let f: Result<File, std::io::Error> = File::create(img_path);

            match f {
                Ok(mut buffer) => {
                    let res: Result<(), std::io::Error> = buffer.write_all(i);

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
                }
            }
        }
    }

    /*
     * Start of image creation, call all functions to create an image
     * 
     */
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
                    if create_dir(&mut relative_path, id) {
                        create_image(&path_clone, &i.bytes().await?);
                    } else {
                        eprintln!("Something went wrong while creating directories, couldn't register image.");
                    }
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
    async fn send_url_file(client: &Client, options: &parse::OptionUser, path: &str,) -> Result<(), Error> {
        let res: Result<Response, Error> = client.get(path)
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
                        doc: &Document, hmap_url: &mut HashMap<String, bool>) {
        let img_dom: Find<'_, Name<&str>> = doc.find(Name("img"));

        for f in img_dom.filter_map(|f| f.attr("src")) {
            if !f.starts_with("http") {
                let split: Vec<_> = options.url.split("/").collect();
                let new_url: String;

                if !f.starts_with("/") {
                    new_url = String::from(split[0]) + "//" + split[2] + "/" + f;
                } else {
                    new_url = String::from(split[0]) + "//" + split[2] + f;
                }
                crawl::try_insert_hmap(hmap_url, options, &new_url, true);
                let _ = send_url_file(cli, options, &new_url).await;
            } else {
                    crawl::try_insert_hmap(hmap_url, options, &f.to_owned(), true);
                    let _ = send_url_file(cli, options, &f).await;
            }
        }
    }
}