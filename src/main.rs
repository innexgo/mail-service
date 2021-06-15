#![feature(async_closure)]
#![feature(never_type)]
use clap::Clap;
// use rusoto_core::Region;
// use rusoto_ses::{Body, Content, Destination, Message, SendEmailRequest, SesClient};
use postgres::{Client, NoTls};
use std::sync::Arc;

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
async fn main() {
  let Opts { database_url, port } = Opts::parse();

  let db: Db = Arc::new(Mutex::new(Client::connect(&database_url, NoTls).unwrap()));

  let api = mail_api::api(db);

  warp::serve(api).run(([127, 0, 0, 1], port)).await;
}

// // Email stuff to think about
// let sc = SesClient::new(Region::default());
//
// async fn send(sc: SesClient, send_address: String, to: String, subject: String, body: String) {
//   sc.send_email(SendEmailRequest {
//     source: send_address,
//     destination: Destination {
//       bcc_addresses: None,
//       cc_addresses: None,
//       to_addresses: Some(vec![to]),
//     },
//     message: Message {
//       subject: subject,
//       body: Body {
//         html: Some(body),
//         text: None,
//       },
//     },
//   })
// }

