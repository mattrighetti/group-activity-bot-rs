mod chat_server;

use chrono::prelude::*;
use tokio_stream::wrappers::UnboundedReceiverStream;
use crate::chat_server::ChatServer;
use chat_server::PrettyPrint;

use lazy_static::lazy_static;
use teloxide::{Bot, prelude::*, requests::ResponseResult, types::{MessageKind::Common}, utils::command::BotCommand};

lazy_static! {
    static ref START_DATE: String = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
    static ref CHAT_SERVER: ChatServer = ChatServer::new();
}

#[derive(BotCommand)]
#[command(rename = "lowercase")]
enum Command {
    #[command(description = "Get users activity.")]
    GroupStats,
    #[command(description = "Get specific user activity")]
    UserStats(String)
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn handle_message(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> ResponseResult<Message> {
    let chat_id = cx.chat_id();
        
    if chat_id > 0 {
        return cx.answer("This bot is only useful in groups.").await
    }

    match cx.update.text() {
        None => Ok(cx.update),
        Some(text) => {
            if let Ok(command) = Command::parse(text, "") {
                match command {
                    Command::GroupStats => {
                        if let (Some(stats), Some(tot)) = (CHAT_SERVER.get_percent(cx.chat_id()), CHAT_SERVER.get_total(cx.chat_id())) {
                            cx.answer(format!("Since {}\nTotal Messages: {}\n\n{}", START_DATE.clone(), tot, stats.pretty_print())).await
                        } else {
                            cx.answer("You first have to write some messages...").await
                        }
                    }
                    Command::UserStats(username) => {
                        if let Some(stats) = CHAT_SERVER.get_percent(cx.chat_id()) {
                            if stats.contains_key(&username) {
                                cx.answer(format!("{}: {:.2}%", &username, stats.get(&username).unwrap()).as_str()).await
                            } else if username.is_empty() {
                                cx.answer("You did not indicate any user, use /userstats <username>").await
                            } else {
                                cx.answer("User {} does not exist or has not written anything in here...").await
                            }
                        } else {
                            cx.answer("You first have to write some messages...").await
                        }
                    }
                }
            } else {
                match cx.update.kind {
                    Common(ref common_msg) => {
                        if let Some(ref user) = common_msg.from {
                            if let Some(ref username) = user.username {
                                CHAT_SERVER.increment(cx.chat_id(), username.clone());
                            }
                        }
                        Ok(cx.update)
                    }
                    _ => Ok(cx.update)
                }
            }
        }
    }
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting group-activity-bot");

    let bot = Bot::from_env().auto_send();



    Dispatcher::new(bot)
    .messages_handler(handle_message_query)
    .dispatch()
    .await;
}

async fn handle_message_query(rx: DispatcherHandlerRx<AutoSend<Bot>, Message>) {
    UnboundedReceiverStream::new(rx)
    .for_each_concurrent(None, |cx| async move {
        handle_message(cx).await
            .expect("Something wrong happened!");
    }).await;
}