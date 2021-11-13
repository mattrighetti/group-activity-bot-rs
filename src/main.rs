use tokio_stream::wrappers::UnboundedReceiverStream;
use teloxide::{
    Bot, 
    adaptors::AutoSend, 
    dispatching::{
        Dispatcher, 
        DispatcherHandlerRx
    }, 
    prelude::{
        RequesterExt, 
        StreamExt
    }, 
    types::Message
};

mod handler;
mod chat_server;

use handler::handle;


#[tokio::main]
async fn main() {
    run().await;
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
        handle(cx).await.expect("Something wrong happened!");
    }).await;
}