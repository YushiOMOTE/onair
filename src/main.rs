use actix_files::NamedFile;
use actix_web::{middleware::Logger, web, App, HttpServer, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct State {
    onair: bool,
}

#[derive(Serialize, Deserialize)]
struct Update {
    onair: bool,
}

impl State {
    fn new() -> Self {
        Self { onair: false }
    }
}

async fn update(
    state: web::Data<Mutex<State>>,
    req: web::Json<Update>,
) -> Result<web::Json<Update>> {
    state.lock().unwrap().onair = req.onair;
    Ok(req)
}

async fn show(state: web::Data<Mutex<State>>) -> Result<NamedFile> {
    let path = if state.lock().unwrap().onair {
        "static/onair.html"
    } else {
        "static/offline.html"
    };
    Ok(NamedFile::open(path)?)
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'p', long = "port")]
    port: u16,
    #[clap(short = 'd', long = "debug")]
    debug: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let level = if args.debug { "debug" } else { "info" };
    std::env::set_var("RUST_LOG", format!("actix_web={}", level));

    env_logger::init();

    let data = web::Data::new(Mutex::new(State::new()));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(data.clone())
            .route("/onair", web::get().to(show))
            .route("/onair", web::post().to(update))
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}
