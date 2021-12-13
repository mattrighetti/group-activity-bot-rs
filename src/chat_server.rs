use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::fmt::Display;
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

trait IntoOrderedVec<T> {
    fn into_ordered_vec(&self) -> Vec<(&String, &T)>;
}

impl<T> IntoOrderedVec<T> for HashMap<String, T>
where
    T: PartialOrd,
{
    fn into_ordered_vec(&self) -> Vec<(&String, &T)> {
        let mut vec: Vec<(&String, &T)> = self.iter().collect();
        vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(Equal));

        vec
    }
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
        self.into_ordered_vec().pretty_print()
    }
}

impl<T> PrettyPrint for Vec<(&String, &T)>
where
    T: Display,
{
    fn pretty_print(&self) -> String {
        let mut builder = String::new();
        for (index, tuple) in self.iter().enumerate() {
            if index == 0 {
                builder.push_str("🥇 ");
            } else if index == 1 {
                builder.push_str("🥈 ");
            } else if index == 2 {
                builder.push_str("🥉 ")
            }

            builder.push_str(format!("{} {:.2}\n", tuple.0, tuple.1).as_str());
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
                *group_hashmap.get_mut(username).unwrap() += 1;
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

    #[test]
    fn test_into_percent() {
        let mut hashmap: HashMap<String, i64> = HashMap::new();
        hashmap.insert("Mario".to_string(), 10);
        hashmap.insert("Fausto".to_string(), 10);
        hashmap.insert("Augusto".to_string(), 129);

        let percent_hashmap = hashmap.into_percent();
        assert_eq!(
            percent_hashmap.get("Mario").unwrap().to_owned(),
            10.0 / 149.0 * 100.0
        );
        assert_eq!(
            percent_hashmap.get("Fausto").unwrap().to_owned(),
            10.0 / 149.0 * 100.0
        );
        assert_eq!(
            percent_hashmap.get("Augusto").unwrap().to_owned(),
            129.0 / 149.0 * 100.0
        );
    }

    #[test]
    fn test_into_ordered_vec() {
        let mut hashmap: HashMap<String, i64> = HashMap::new();
        hashmap.insert("Mario".to_string(), 10);
        hashmap.insert("Fausto".to_string(), 12);
        hashmap.insert("Augusto".to_string(), 129);

        let ord_vec = hashmap.into_ordered_vec();
        assert_eq!(ord_vec[0].0, &"Augusto".to_string());
        assert_eq!(ord_vec[1].0, &"Fausto".to_string());
        assert_eq!(ord_vec[2].0, &"Mario".to_string());
    }
}
