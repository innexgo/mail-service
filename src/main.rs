#![feature(async_closure)]
use clap::Clap;
use rusqlite::Connection;
use std::sync::Arc;
//use std::sync::Mutex;

use tokio::sync::Mutex;

mod utils;

// db web stuff
mod log_api;
mod log_db_types;
mod log_handlers;

// database interface
mod event_service;

static SERVICE_NAME: &str = "log-service";

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

  let api = log_api::api(db);

  warp::serve(api).run(([127, 0, 0, 1], port)).await;
}
