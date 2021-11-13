use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type ChatDatabase = HashMap<i64, HashMap<String, i64>>;

#[derive(Debug)]
pub struct ChatServer {
    pub database: Arc<Mutex<ChatDatabase>>,
}

trait IntoPercent {
    fn into_percent(&self) -> HashMap<String, f32>;
}

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

impl IntoPercent for HashMap<String, i64> {
    fn into_percent(&self) -> HashMap<String, f32> {
        let tot: i64 = self.values().sum();
        let mut result = HashMap::new();
        for (k, v) in self {
            result.insert(k.clone(), *v as f32 / tot as f32 * 100.0);
        }
        
        result
    }
}

impl PrettyPrint for HashMap<String, f32> {
    fn pretty_print(&self) -> String {
        let mut builder = String::new();
        for (k, v) in self {
            builder.push_str(format!("{}: {:.2}%\n", k, v).as_str());
        }
        
        builder
    }
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            database: Arc::new(Mutex::new(ChatDatabase::new())),
        }
    }

    pub fn increment(&self, chat_id: i64, username: &String) {
        let mut lock = self.database.lock().unwrap();

        if let Some(group_hashmap) = lock.get_mut(&chat_id) {
            if group_hashmap.get(username).is_some() {
                let old = group_hashmap.get(username).unwrap().to_owned();
                group_hashmap.insert(username.clone(), old+1);
            } else {
                group_hashmap.insert(username.clone(), 1);
            }
        } else {
            let mut hash = HashMap::new();
            hash.insert(username.clone(), 1);
            lock.insert(chat_id, hash);
        }
    }

    pub fn get_percent(&self, chat_id: i64) -> Option<HashMap<String, f32>> {
        let lock = self.database.lock().unwrap();

        if let Some(group_hashmap) = lock.get(&chat_id) {
            Some(group_hashmap.into_percent())
        } else {
            None
        }
    }

    pub fn get_total(&self, chat_id: i64) -> Option<i64> {
        let lock = self.database.lock().unwrap();

        if let Some(group_hashmap) = lock.get(&chat_id) {
            Some(group_hashmap.values().sum())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn into_percent() {
        let mut hashmap: HashMap<String, i64> = HashMap::new();
        hashmap.insert("Mario".to_string(), 10);
        hashmap.insert("Fausto".to_string(), 10);
        hashmap.insert("Augusto".to_string(), 129);

        let percent_hashmap = hashmap.into_percent();
        println!("{:?}", percent_hashmap);
        assert_eq!(percent_hashmap.get("Mario").unwrap().to_owned(), 10.0 / 149.0 * 100.0);
        assert_eq!(percent_hashmap.get("Fausto").unwrap().to_owned(), 10.0 / 149.0 * 100.0);
        assert_eq!(percent_hashmap.get("Augusto").unwrap().to_owned(), 129.0 / 149.0 * 100.0);
    }
}