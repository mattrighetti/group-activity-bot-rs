mod chat_server;

use crate::chat_server::ChatServer;
use chat_server::PrettyPrint;

use lazy_static::lazy_static;
use teloxide::{Bot, prelude::*, types::MessageKind::Common};

lazy_static! {
    static ref CHAT_SERVER: ChatServer = ChatServer::new();
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, move |message| async move {
        if message.update.text().unwrap() == "group_stats" {
            if let Some(stats) = CHAT_SERVER.get_percent(message.chat_id()) {
                message.answer(stats.pretty_print()).await?;
            } else {
                message.answer("You first have to write some messages...").await?;
            }
        } else {
            match message.update.kind {
                Common(ref common_msg) => {
                    if let Some(ref user) = common_msg.from {
                        if let Some(ref username) = user.username {
                            println!("user {:?} in chat {:?} just sent a message", username, message.chat_id());
                            CHAT_SERVER.increment(message.chat_id(), username.clone())
                        }
                    }
                }
                _ => {}
            }
        }
        respond(())
    })
    .await;
}