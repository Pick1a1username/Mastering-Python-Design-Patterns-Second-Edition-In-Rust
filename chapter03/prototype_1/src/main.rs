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
        let mut summary = vec![format!("Website: \"{}\"\n", self.info["name"])];

        let infos = self.info.clone();
        // HashMap cannot be sorted like Python.
        // So, get sorted keys of it separately and print infos orderly.
        // Reference: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.iter
        let sorted_keys = sorted_keys(&infos);

        for key in sorted_keys {
            if key == String::from("name") {
                continue
            }
            summary.push(format!("{}: {}\n", key, infos[&key]));
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

                // If a key already exists, the value will be overwritten.
                for (new_key, new_value) in new_infos {
                    new_obj.info.insert(new_key.clone(), new_value.clone());
                }

                Ok(new_obj)
            },
            None => Err(format!("Incorrect object identifier: {}", identifier)),
        }
    }
}


fn sorted_keys(hashmap: &HashMap<String, String>) -> Vec<String> {
    // Get keys
    let mut keys: Vec<String> = Vec::new();
    
    for key in hashmap.keys() {
        keys.push(key.clone());
    }

    // Sort and return key vector
    keys.sort();
    keys
}
 
fn main() {
    // Since HashMap can have the type of value specified in advance,
    // keywords will be set as String comma-breaked.
    let keywords: String = String::from("python,data,apis,automation");
    let mut keywords_mapped: HashMap<String, String> = HashMap::new();
    keywords_mapped.insert(String::from("keywords"), keywords);
    keywords_mapped.insert(String::from("category"), String::from("Blog"));

    let site1 = Website::new("ContentGardening".to_string(),
        "contentgardening.com".to_string(),
        "Automation and data-driven apps".to_string(),
        "Kamon Ayeva".to_string(),
        keywords_mapped
    );

    let mut prototype = Prototype::new();
    let identifier = "ka-cg-1".to_string();
    prototype.register(identifier, site1);

    // println!("{}", prototype.objects[&"ka-cg-1".to_string()].get_info());

    // Prepare to clone site1 with addtional infos
    let mut site2_add_info: HashMap<String, String> = HashMap::new();
    site2_add_info.insert("name".to_string(), "ContentGardeningPlayground".to_string());
    site2_add_info.insert("domain".to_string(), "play.contentgardening.com".to_string());
    site2_add_info.insert("description".to_string(), "Experimentation for techniques featured on the blog".to_string());
    site2_add_info.insert("category".to_string(), "Membership site".to_string());
    site2_add_info.insert("creation_date".to_string(), "2018-08-01".to_string());

    // Clone site1 with addtional infos.
    let site2 = prototype.clone("ka-cg-1".to_string(), site2_add_info).unwrap();

    println!("{}", prototype.objects[&"ka-cg-1".to_string()].get_info());
    println!("{}", site2.get_info());

    // https://stackoverflow.com/questions/30157258/does-rust-track-unique-object-ids-and-can-we-print-them
    println!("Address of site1 in prototype : {:p} != Address of site2 : {:p}", &prototype.objects[&"ka-cg-1".to_string()], &site2);
}
