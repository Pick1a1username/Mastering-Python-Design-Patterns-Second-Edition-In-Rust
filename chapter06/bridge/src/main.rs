use std::fs;

extern crate reqwest;

struct ResourceContent {
    imp: Box<ResourceContentFetcher>,
}

impl ResourceContent {
    fn show_content(&self, path: String) {
        self.imp.fetch(path)
    }
}

trait ResourceContentFetcher {
    fn fetch(&self, path: String);
}

struct URLFetcher {}

impl ResourceContentFetcher for URLFetcher {
    fn fetch(&self, path: String) {
        let mut response = reqwest::get(&path).unwrap();

        if response.status().is_success() {
            let the_page = response.text().unwrap();
            println!("{}", the_page);
        }
    }
}

struct LocalFileFetcher {}

impl ResourceContentFetcher for LocalFileFetcher {
    fn fetch(&self, path: String) {
        println!("{}"
            , fs::read_to_string(path).unwrap()
        );
    }
}



fn main() {
    let url_fetcher = URLFetcher {};
    let iface = ResourceContent { imp: Box::new(url_fetcher) };
    iface.show_content("http://python.org".to_string());

    println!("===================");

    let localfs_fetcher = LocalFileFetcher {};
    let iface = ResourceContent { imp: Box::new(localfs_fetcher) };
    iface.show_content("file.txt".to_string());
}
