// https://stackoverflow.com/questions/27221504/how-can-you-make-a-safe-static-singleton-in-rust
// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html
// https://docs.rs/reqwest/0.9.17/reqwest/

extern crate reqwest;

struct URLFetcher {
    urls: Vec<String>,
}

impl URLFetcher {
    fn new() -> URLFetcher {
        URLFetcher { urls: Vec::new() }
    }

    fn fetch(&mut self, url: String) -> Result<(), reqwest::Error> {
        let mut response = reqwest::get(&url)?;

        if response.status().is_success() {
            let the_page = response.text()?;
            println!("{}", the_page);

            self.urls.push(url);
        }
        Ok(())
    }

    fn dump_url_registry(&self) -> String {
        let mut dump = String::new();

        for entry in &self.urls {
            dump.push_str(&entry);
            dump.push_str(",");
        }

        dump
    }
} 

fn main() {
    let my_urls = vec!["http://www.voidspace.org.uk", 
        "http://google.com", 
        "http://python.org",
        "https://www.python.org/error",
    ];

    // print(URLFetcher() is URLFetcher())

    let mut fetcher = URLFetcher::new();

    for url in my_urls {
        fetcher.fetch(url.to_string());
    }





}
