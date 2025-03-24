pub mod document {
    use std::collections::HashMap ;
    use reqwest::Client;
    use select::document::{ Document, Find };
    use select::predicate::Name;
    use regex::Regex;
    use crate::parse_flags::parse::OptionUser;
    use crate::client::{ crawl, image };
  
    fn  get_links(options: &OptionUser, doc: &Document,
        hmap_url: &mut HashMap<String, bool>) -> bool {
        let reg_str: String = String::from("^(https?://") + options.website_name.as_str() + ")*";
        let regex: Result<Regex, regex::Error> = Regex::new(reg_str.as_str());
        let a_dom: Find<'_, Name<&str>> = doc.find(Name("a"));
        let mut find_input: bool = false;

        a_dom.filter_map(|f| f.attr("href"))
        .for_each(|f| {
            find_input = true;
            if !f.starts_with("http") {
                match &regex {
                    Ok(reg) => {
                        let res_captures: Option<regex::Captures<'_>> = reg.captures(&options.url);
                        if let Some(capture) = res_captures {
                            let res_url = capture.get(0);
                            if let Some(url_str) = res_url {
                                let new_url: String;

                                if !f.starts_with("/") {
                                    new_url = String::from(url_str.as_str()) + "/" + f;
                                } else {
                                    new_url = String::from(url_str.as_str()) + f;
                                }
                                crawl::try_insert_hmap(&regex, hmap_url, &new_url, false);
                            } else {
                                eprintln!("Url parsing is wrong, is url from the crawled website?");
                            }
                        } else {
                            eprintln!("Url parsing is wrong, is url from the crawled website?");
                        }
                    },
                    Err(_) => {},
                };
            } else {
                crawl::try_insert_hmap(&regex, hmap_url, &f.to_owned(), false);
            }
        });
        find_input
    }

    pub async fn parse_doc(options: &OptionUser, cli: &Client,
        text: &String, hmap_url: &mut HashMap<String, bool>) -> bool {
        let doc: Document = Document::from(text.as_str());
        let find_input: bool = get_links(&options, &doc, hmap_url);

        image::img::get_images(options, cli, &doc, hmap_url).await;
        find_input
    }
}