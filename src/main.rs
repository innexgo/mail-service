#![feature(async_closure)]
use clap::Clap;
use rusqlite::Connection;
use std::sync::Arc;

use tokio::sync::Mutex;

mod utils;

// db web stuff
mod mail_db_types;
mod mail_service;
mod mail_api;
mod mail_handlers;

static SERVICE_NAME: &str = "mail-service";

#[derive(Clap, Clone)]
struct Opts {
  #[clap(short, long)]
  database_url: String,
  #[clap(short, long)]
  port: u16,
}

pub type Db = Arc<Mutex<Connection>>;

#[tokio::main]
async fn main() {
  let Opts {
    database_url,
    port,
  } = Opts::parse();

  let db: Db = Arc::new(Mutex::new(Connection::open(database_url).unwrap()));

  let api = mail_api::api(db);

  warp::serve(api).run(([127, 0, 0, 1], port)).await;
}
