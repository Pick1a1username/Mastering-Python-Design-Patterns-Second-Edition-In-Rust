use std::collections::HashMap;

// Since there's no feature like '**kwargs' of Python in Rust,
// and additional fields cannot be add to defined struct,
// HashMap is used instead in this example.
#[derive(Clone)]
struct Website {
    info: HashMap<String, String>,
}
 
impl Website {
    fn new(name: String, domain: String, description: String, author: String, add_info: HashMap<String, String>) -> Website {
        // Examples of optional attributes (add_info):
        // category, creation_date, technologies, keywords.
        let mut default_values = HashMap::new();

        default_values.insert(String::from("name"), name);
        default_values.insert(String::from("domain"), domain);
        default_values.insert(String::from("description"), description);
        default_values.insert(String::from("author"), author);

        for (key, value) in &add_info {
            default_values.insert(String::from(key.clone()), value.clone());
        }

        Website { info: default_values }
    }

    fn get_info(&self) -> String {
        let mut summary = vec![format!("Website \"{}\"", self.info["name"])];

        let mut infos = self.info.clone();
        sort_string_only_hashmap(&mut infos);

        for (key, value) in infos {
            if key == String::from("name") {
                continue
            }
            summary.push(format!("{}: {}\n", key, value));
        }

        let mut summary_for_print = String::new();

        for entry in summary {
            summary_for_print.push_str(&entry);
        }

        summary_for_print
    }
}

struct Prototype {
    objects: HashMap<String, Website>,
}

impl Prototype {
    fn new() -> Prototype {
        Prototype {
            objects: HashMap::new(),
        }
    }

    fn register(&mut self, identifier: String, obj: Website) {
        self.objects.insert(identifier, obj);
    }

    fn unregister(&mut self, identifier: String) {
        self.objects.remove(&identifier);
    }

    fn clone(&mut self, identifier: String, new_infos: HashMap<String, String>) -> Result<Website, String>  {
        let found = self.objects.get(&identifier);

        match found {
            Some(current_obj) => {
                let mut new_obj = current_obj.clone();

                // Todo: If the key already exists, it should be overwritten.
                for (new_key, new_value) in new_infos {
                    new_obj.info.insert(new_key.clone(), new_value.clone());
                }

                Ok(new_obj)
            },
            None => Err(format!("Incorrect object identifier: {}", identifier)),
        }
    }
}


fn sort_string_only_hashmap(hashmap: &mut HashMap<String, String>) {
    // Get keys
    let mut keys: Vec<String> = Vec::new();
    
    for key in hashmap.keys() {
        keys.push(key.clone());
    }

    // Sort key vector
    keys.sort();
 
    // Recreate hashmap
    let mut sorted_hashmap: HashMap<String, String> = HashMap::new();
    for key in keys {
        sorted_hashmap.insert(key.clone(), hashmap[&key].clone());
    }

    // Set the variable to the recreated hashmap
    *hashmap = sorted_hashmap;
}

fn main() {
    // Since HashMap can have the type of value specified in advance,
    // keywords will be set as String comma-breaked.
    let keywords: String = String::from("python,data,apis,automation");
    let mut keywords_mapped: HashMap<String, String> = HashMap::new();
    keywords_mapped.insert(String::from("keywords"), keywords);

    let site1 = Website::new("ContentGardening".to_string(),
        "contentgardening.com".to_string(),
        "Automation and data-driven apps".to_string(),
        "Kamon Ayeva".to_string(),
        keywords_mapped
    );

    let mut prototype = Prototype::new();
    let identifier = "ka-cg-1".to_string();
    prototype.register(identifier, site1);

    // 
}
