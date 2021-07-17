use std::collections::HashMap;

pub struct KvStore {
    items: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.items.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        let val = self.items.get(key.as_str());
        match val {
            Some(v) => Some(v.to_string()),
            _ => None,
        }
    }

    pub fn remove(&mut self, key: String) {
        self.items.remove(key.as_str());
    }
}

fn main() {
    let mut store = KvStore::new();
    store.set("foo".to_string(), "bar".to_string());
    if let Some(v) = store.get("foo".to_string()) {
        println!("Got {}", v);
    }
}
