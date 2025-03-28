pub mod document {
    use std::collections::HashMap ;
    use reqwest::Client;
    use select::document::{ Document, Find };
    use select::predicate::Name;
    use crate::parse_flags::parse::OptionUser;
    use crate::client::{ crawl, image };
  
    fn  get_links(options: &OptionUser, doc: &Document,
        hmap_url: &mut HashMap<String, bool>) -> bool {
        let a_dom: Find<'_, Name<&str>> = doc.find(Name("a"));
        let mut find_input: bool = false;

        a_dom.filter_map(|f| f.attr("href"))
        .for_each(|f| {
            find_input = true;
            if !f.starts_with("http") {
                let split: Vec<_> = options.url.split("/").collect();
                let new_url: String;

                if !f.starts_with("/") {
                    new_url = String::from(split[0]) + "//" + split[2] + "/" + f;
                } else {
                    new_url = String::from(split[0]) + "//" + split[2] + f;
                }
                crawl::try_insert_hmap(hmap_url, options, &new_url, false);
            } else {
                crawl::try_insert_hmap(hmap_url, options, &f.to_owned(), false);
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