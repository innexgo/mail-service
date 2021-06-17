use clap::Clap;
// use rusoto_core::Region;
// use rusoto_ses::{Body, Content, Destination, Message, SendEmailRequest, SesClient};
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};

use tokio::sync::Mutex;

mod utils;

// db web stuff
mod mail_api;
mod mail_db_types;
mod mail_handlers;
mod mail_service;

static SERVICE_NAME: &str = "mail-service";

#[derive(Clap, Clone)]
struct Opts {
  #[clap(short, long)]
  database_url: String,
  #[clap(short, long)]
  port: u16,
}

pub type Db = Arc<Mutex<Client>>;

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
  let Opts { database_url, port } = Opts::parse();

  let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

  // The connection object performs the actual communication with the database,
  // so spawn it off to run on its own.
  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("connection error: {}", e);
    }
  });

  let db: Db = Arc::new(Mutex::new(client));

  let api = mail_api::api(db);

  warp::serve(api).run(([127, 0, 0, 1], port)).await;

  Ok(())
}
