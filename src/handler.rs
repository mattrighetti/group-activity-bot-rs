use anyhow::Result;
use chrono::Local;
use crate::chat_server::{
    ChatServer,
    PrettyPrint
};
use teloxide::{
    prelude::*, 
    types::MessageKind::Common, 
    utils::command::BotCommand
};

use lazy_static::lazy_static;


lazy_static! {
    static ref START_DATE: String = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
    static ref CHAT_SERVER: ChatServer = ChatServer::new();
}


#[derive(BotCommand)]
#[command(rename = "lowercase")]
enum Command {
    GroupStats,
    UserStats(String),
}

pub async fn handle(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> Result<()> {
    let chat_id = cx.chat_id();
    
    // Telegram uses negative numbers for groups' `chat_id`
    if chat_id > 0 {
        answer(&cx, "This bot is only useful in groups.").await?
    }

    let text = match cx.update.text() {
        Some(text) => text,
        None => { 
            return Ok(()); 
        }
    };

    let mut response = String::from("");
    if let Ok(command) = Command::parse(text, "gactivitybot") {
        response = match command {
            Command::GroupStats => get_group_stats(chat_id),
            Command::UserStats(username) => get_user_stats(chat_id, username)
        }
    } else {
        match &cx.update.kind {
            Common(common_msg) => {
                if let Some(user) = &common_msg.from {
                    if let Some(username) = &user.username {
                        CHAT_SERVER.increment(chat_id, &username);
                    }
                }
            }
            _ => {}
        }
    }

    answer(&cx, response.as_str()).await?;

    Ok(())
}

async fn answer(cx: &UpdateWithCx<AutoSend<Bot>, Message>, answer: &str) -> Result<()> {
    if !answer.is_empty() {
        cx.answer(answer).await?;
    }

    Ok(())
}

fn get_user_stats(group_id: i64, username: String) -> String {
    if let Some(stats) = CHAT_SERVER.get_percent(group_id) {
        if username.is_empty() {
            "You did not indicate any user, use /userstats <username>".to_string()
        } else if stats.contains_key(&username) {
            format!("{}: {:.2}%", &username, stats.get(&username).unwrap())
        } else {
            format!("User {} does not exist or has not written anything in here...", username)
        }
    } else {
        "You first have to write some messages...".to_string()
    }
}

fn get_group_stats(group_id: i64) -> String {
    if let (Some(stats), Some(total)) = (CHAT_SERVER.get_percent(group_id), CHAT_SERVER.get_total(group_id)) {
        format!("Since {}\nTotal Messages: {}\n\n{}", START_DATE.clone(), total, stats.pretty_print())
    } else {
        "You first have to write some messages...".to_string()
    }
}

