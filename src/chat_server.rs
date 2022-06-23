use std::sync::{Arc, Mutex};
use rusqlite::{Connection, params, Result};

use crate::db::get_db;


#[derive(Debug)]
pub struct ChatServer {
    pub database: Arc<Mutex<Connection>>
}

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

impl PrettyPrint for Vec<Data> {
    fn pretty_print(&self) -> String {
        let mut builder = String::new();
        for (index, data) in self.iter().enumerate() {
            if index == 0 {
                builder.push_str("🥇 ");
            } else if index == 1 {
                builder.push_str("🥈 ");
            } else if index == 2 {
                builder.push_str("🥉 ")
            }

            builder.push_str(format!("{} {:.2}%\n", data.username, data.percent).as_str());
        }
        builder.push_str(format!("\n#stats").as_str());

        builder
    }
}

#[derive(Debug, PartialEq)]
struct Data {
    username: String,
    percent: f32,
}

impl Data {
    fn percent_str(&self) -> String {
        format!("{:.2}", self.percent)
    }
}

impl ChatServer {
    pub fn new(db_path: String) -> Self {
        let conn = get_db(Some(db_path.as_str())).unwrap();

        ChatServer {
            database: Arc::new(Mutex::new(conn))
        }
    }

    pub fn in_memory() -> Self {
        let conn = get_db(None).unwrap();

        ChatServer {
            database: Arc::new(Mutex::new(conn))
        }
    }

    pub fn store_msg(&self, chat_id: i64, username: &String) -> Result<()> {
        let lock = self.database.lock().unwrap();
        let mut stmt = lock.prepare("INSERT INTO message_records(group_id, username) VALUES (?, ?)")?;
        stmt.execute(params![chat_id, username])?;

        Ok(())
    }

    pub fn get_tot_msg(&self, chat_id: i64) -> Result<i64> {
        let lock = self.database.lock().unwrap();
        let mut stmt = lock.prepare(
            "SELECT count(*)
            from message_records
            where group_id = ?;"
        )?;

        let tot = stmt.query_row([chat_id], |row| Ok(row.get(0)?)).unwrap();

        Ok(tot)
    }

    fn get_group_percent(&self, chat_id: i64) -> Result<Vec<Data>> {
        let lock = self.database.lock().unwrap();
        let mut stmt = lock.prepare(
            "SELECT username, count(*) * 100.0/ sum(count(*)) over () as percent
            from message_records
            where group_id = ?
            group by group_id, username
            order by percent desc;"
        )?;

        let percents_iter = stmt.query_map([chat_id], |row| {
            Ok(Data { username: row.get(0)?, percent: row.get(1)? })
        }).unwrap();

        let perc_vec: Vec<Data> = percents_iter.map(|d| { d.unwrap() }).collect();

        Ok(perc_vec)
    }

    pub fn get_group_percent_str(&self, chat_id: i64) -> Result<String> {
        let data = self.get_group_percent(chat_id)?;
        
        Ok(data.pretty_print())
    }

    pub fn get_user_percent_str(&self, chat_id: i64, username: &String) -> Result<String> {
        let lock = self.database.lock().unwrap();
        let mut stmt = lock.prepare(
            "SELECT mr.username, count(*) * 100.0 / (select count(*) from message_records where group_id = mr.group_id) as percent
            from message_records as mr
            where mr.group_id = ? and mr.username = ?;"
        )?;

        let data = stmt.query_row(params![chat_id, username], |row| {
            Ok(Data { username: row.get(0)?, percent: row.get(1)? })
        })?;

        Ok(data.percent_str() + "%")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion() {
        let cs = ChatServer::in_memory();

        for _ in 0..10 {
            cs.store_msg(0, &format!("{}", 0)).unwrap();
        }

        for _ in 0..3 {
            cs.store_msg(0, &format!("{}", 1)).unwrap();
        }

        for _ in 0..7 {
            cs.store_msg(0, &format!("{}", 2)).unwrap();
        }

        cs.store_msg(0, &format!("{}", 3)).unwrap();

        let tot: f32 = 10.0 + 3.0 + 7.0 + 1.0;
        let data = cs.get_group_percent(0).unwrap();
        
        assert_eq!(data[0].percent_str(), format!("{:.2}", 10.0/tot * 100.0));
        assert_eq!(data[1].percent_str(), format!("{:.2}", 7.0/tot  * 100.0));
        assert_eq!(data[2].percent_str(), format!("{:.2}", 3.0/tot  * 100.0));
        assert_eq!(data[3].percent_str(), format!("{:.2}", 1.0/tot  * 100.0));
    }

    #[test]
    fn test_get_user_percent() {
        let cs = ChatServer::in_memory();

        for _ in 0..10 {
            cs.store_msg(0, &"0".to_string()).unwrap();
        }

        for _ in 0..3 {
            cs.store_msg(0, &"1".to_string()).unwrap();
        }

        for _ in 0..7 {
            cs.store_msg(0, &"2".to_string()).unwrap();
        }

        cs.store_msg(0, &"3".to_string()).unwrap();

        let tot: f32 = 10.0 + 3.0 + 7.0 + 1.0;
        let percent = cs.get_user_percent_str(0, &String::from("1")).unwrap();

        assert_eq!(percent, format!("{:.2}", 3.0/tot * 100.0));
    }
}
