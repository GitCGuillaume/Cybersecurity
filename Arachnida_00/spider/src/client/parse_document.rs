pub mod document {
    use std::collections::HashMap ;
    use reqwest::Client;
    use select::document::{ Document, Find };
    use select::predicate::Name;
    use regex::Regex;
    use crate::parse_flags::parse::OptionUser;
    use crate::client::{ crawl, image };
  
    fn  get_links(_url: &String, _doc: &Document,
        hmap_url: &mut HashMap<String, bool>) -> bool {
        let regex: Result<Regex, regex::Error> = Regex::new(r"^https?://[\w\d.-]*");
        let a_dom: Find<'_, Name<&str>> = _doc.find(Name("a"));
        let mut find_input: bool = false;
    
        a_dom.filter_map(|f| f.attr("href"))
        .for_each(|f| {
            find_input = true;
            if !f.starts_with("http") {
                match &regex {
                    Ok(reg) => {
                        let captures = reg.captures(_url)
                                            .unwrap().get(0).unwrap().as_str();
                        let new_url: String;

                        if !f.starts_with("/") {
                            new_url = String::from(captures) + "/" + f;
                            println!("{new_url}");
                        } else {
                            new_url = String::from(captures) + f;
                            println!("{new_url}");
                        }
                        crawl::try_insert_hmap(hmap_url, &new_url, false);
                    },
                    Err(_) => {},
                };
            } else {
                crawl::try_insert_hmap(hmap_url, &f.to_owned(), false);
            }
        });
        find_input
    }

    pub async fn parse_doc(options: &OptionUser, cli: &Client,
        text: &String, hmap_url: &mut HashMap<String, bool>) -> bool {
        let doc: Document = Document::from(text.as_str());
        let mut _find_input: bool = false;
    
        _find_input = get_links(&options.url, &doc, hmap_url);
        image::img::get_images(options, cli, &doc, hmap_url).await;
        _find_input
    }
}