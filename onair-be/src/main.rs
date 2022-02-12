use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use poem::{
    endpoint::StaticFilesEndpoint,
    get, handler,
    listener::TcpListener,
    post,
    web::{
        websocket::{Message, WebSocket},
        Data, Json,
    },
    EndpointExt, IntoResponse, Route, Server,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::{self, Sender};
use tracing::*;

type Context = Arc<Mutex<ContextInner>>;

#[derive(Debug)]
struct ContextInner {
    onair: bool,
    sender: Sender<Update>,
}

impl ContextInner {
    fn new() -> Self {
        let sender = broadcast::channel::<Update>(10).0;
        Self {
            onair: false,
            sender,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Update {
    onair: bool,
}

impl Update {
    fn new(onair: bool) -> Self {
        Self { onair }
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// Address to bind
    #[clap(short = 'b', long = "bind", default_value = "0.0.0.0:8080")]
    bind: String,
    /// Enable debug print
    #[clap(short = 'd', long = "debug")]
    debug: bool,
}

#[handler]
fn check(data: Data<&Context>) -> Json<Update> {
    let resp = Update::new(data.lock().unwrap().onair);
    info!("Check {:?}", resp);
    Json(resp)
}

#[handler]
fn update(data: Data<&Context>, update: Json<Update>) -> Json<Update> {
    info!("Update {:?}", update);
    let mut context = data.lock().unwrap();
    context.onair = update.onair;
    let _ = context.sender.send((*update).clone());
    update
}

#[handler]
fn subscribe(data: Data<&Context>, ws: WebSocket) -> impl IntoResponse {
    let mut context = data.lock().unwrap();
    let mut subscriber = context.sender.subscribe();

    ws.on_upgrade(move |sock| async move {
        let (mut tx, _) = sock.split();

        tokio::spawn(async move {
            while let Ok(msg) = subscriber.recv().await {
                if tx
                    .send(Message::Text(serde_json::to_string(&msg).unwrap()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        })
    })
}

#[handler]
fn index() -> String {
    format!("OnAir {}", env!("CARGO_PKG_VERSION"))
}

fn init() -> Args {
    let args = Args::parse();
    let level = if args.debug { "debug" } else { "info" };
    std::env::set_var("RUST_LOG", level);

    tracing_subscriber::fmt::init();

    args
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = init();

    let ctx = Arc::new(Mutex::new(ContextInner::new()));

    let app = Route::new()
        .at("/", get(index))
        .at("/state", post(update).get(check))
        .at("/subscribe", get(subscribe))
        .nest(
            "/view",
            StaticFilesEndpoint::new("./static")
                .show_files_listing()
                .index_file("index.html"),
        )
        .data(ctx);

    Server::new(TcpListener::bind(&args.bind)).run(app).await
}
