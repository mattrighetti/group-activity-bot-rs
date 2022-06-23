pub mod handler;
mod chat_server;
mod db;

use std::{sync::Arc, env};
use teloxide::prelude::*;

use crate::{
    handler::handle,
    chat_server::ChatServer
};


#[tokio::main]
async fn main() {
    let log_path = std::env::var("LOG_PATH").unwrap();
    log4rs::init_file(log_path, Default::default()).unwrap();
    run().await;
}

async fn run() {
    log::info!("Starting group-activity-bot");

    let bot = Bot::from_env().auto_send();
    let db_path = env::var("DB_PATH").unwrap();
    let chat_server = Arc::new(ChatServer::new(db_path));

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handle));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![chat_server])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;

    log::info!("Closing bot... Goodbye!");
}