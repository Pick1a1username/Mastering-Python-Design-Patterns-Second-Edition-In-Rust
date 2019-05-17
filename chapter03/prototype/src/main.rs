use std::collections::HashMap;

// Since there's no feature like '**kwargs' of Python in Rust,
// and additional fields cannot be add to defined struct,
// HashMap is used instead in this example.
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

        String::from("test")
    }
}

fn sort_string_only_hashmap(hashmap: &mut HashMap<String, String>) {
    // Get keys
    let mut keys: Vec<&String> = Vec::new();
    
    for ref key in hashmap.keys() {
        keys.push(key);
    }

    // Sort key vector
    keys.sort();
 
    // Recreate hashmap
    let mut sorted_hashmap: HashMap<String, String> = HashMap::new();
    for ref key in keys {
        sorted_hashmap.insert(key, hashmap[key]);
    }


    // Set the variable to the recreated hashmap
}

fn main() {
    println!("Hello, world!");
}
