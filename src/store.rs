use std::collections::HashMap;
// we need the HashMap type from the standard library because we are going to use it to store our data
use std::time::{Duration, Instant};
// time is for the expiration time of the key-value pair in the store struct below

// this struct Store is a hashmap that maps Strings to Entry structs
// provide resistance against DoS attacks, which is a type of attack that attempts to make a server or network resource unavailable to its intended users
pub struct Store {
    // data: HashMap<String, String>,
    // we use hashmap because it is a fast way to store data, and we can use it to store our key-value pairs
    // here, we are using a hashmap that maps strings to strings, but we will change this to a hashmap that maps strings to Entry structs
    data: HashMap<String, Entry>,
}

// entry struct is a hashmap that maps strings to strings, this is for our key-value store in which we will store our data
struct Entry {
    t: Option<Instant>,
    value: String,
}

// implementation of the Store struct, which is the key-value store, which is a hashmap that maps strings to Entry structs
impl Store {
    pub fn new() -> Self {
        Store {
            data: HashMap::new(),
        }
    }

    // function that sets the key-value pair, without an expiration time, which is the default, so we set the expiration time to None
    pub fn set(&mut self, key: String, value: String) {
        let entry = Entry { t: None, value };
        self.data.insert(key, entry);
    }

    // function that sets the key-value pair with an expiration time
    pub fn set_with_expiry(&mut self, key: String, value: String, expiry_ms: u64) {
      // assigns entry to a new Entry struct with the value of value and the expiration time of Instant::now() + Duration::from_millis(expiry_ms)
        let entry = Entry {
            t: Some(Instant::now() + Duration::from_millis(expiry_ms)),
            // value is the value of the key-value pair
            value,
        };
        // insert the key-value pair into the hashmap, which is the data field of the Store struct, which is the key-value store
        self.data.insert(key, entry);
    }

    // function that gets the value of the key-value pair
    pub fn get(&mut self, key: String) -> Option<String> {
        // self.data.get(key.as_str()).cloned()
        match self.data.get(key.as_str()) {
            Some(entry) => {
                // Lazily expire keys as they are requested
                if let Some(t) = &entry.t {
                    if Instant::now() > t.clone() {
                        self.data.remove(key.as_str());
                        return None;
                    }
                }
                // return the value of the key-value pair
                Some(entry.value.clone())
            }
            // if the key-value pair is not found, return None
            None => None,
        }
    }
}
