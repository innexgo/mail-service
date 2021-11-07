#![feature(async_closure)]
use clap::Parser;
use std::error::Error;
use std::sync::Arc;
use warp::Filter;

use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};

mod utils;

// db web stuff
mod api;
mod db_types;
mod handlers;
mod mail_service;
mod ses_aws;

static SERVICE_NAME: &str = "mail-service";

#[derive(Parser, Clone)]
#[clap(version = "0.1")]
struct Opts {
  /// URL of postgres db to connect to
  #[clap(short, long)]
  database_url: String,
  /// The port to run on
  #[clap(short, long)]
  port: u16,
  /// The email address to send from
  #[clap(short, long)]
  from_address: String,
  /// Instead of sending emails via SES, simply prints them to stdout.
  #[clap(long)]
  dryrun: bool,
}

pub type Db = Arc<Mutex<Client>>;

#[derive(Clone)]
pub struct Config {
  pub from_address: String,
  pub client: Option<aws_sdk_sesv2::Client>,
}

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
  let Opts {
    database_url,
    port,
    from_address,
    dryrun,
  } = Opts::parse();

  // create postgres connection
  let (client, connection) = loop {
    match tokio_postgres::connect(&database_url, NoTls).await {
      Ok(v) => break v,
      Err(e) => utils::log(utils::Event {
        msg: e.to_string(),
        source: e.source().map(|x| x.to_string()),
        severity: utils::SeverityKind::Error,
      }),
    }

    // sleep for 5 seconds
    std::thread::sleep(std::time::Duration::from_secs(5));
  };

  // The connection object performs the actual communication with the database,
  // so spawn it off to run on its own.
  tokio::spawn(async move {
    if let Err(e) = connection.await {
      utils::log(utils::Event {
        msg: e.to_string(),
        source: e.source().map(|x| x.to_string()),
        severity: utils::SeverityKind::Error,
      })
    }
  });

  // resolve aws region

  let db: Db = Arc::new(Mutex::new(client));

  let config = Config {
    from_address,
    client: if dryrun {
      None
    } else {
      Some(ses_aws::build_client().await)
    },
  };

  let api = api::api(db, config);

  let log = warp::log::custom(|info| {
    // Use a log macro, or slog, or println, or whatever!
    utils::log(utils::Event {
      msg: info.method().to_string(),
      source: Some(info.path().to_string()),
      severity: utils::SeverityKind::Info,
    });
  });

  warp::serve(api.with(log)).run(([0, 0, 0, 0], port)).await;

  Ok(())
}
