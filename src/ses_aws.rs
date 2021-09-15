use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sesv2::model::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::{Client, Config};
use aws_types::region::Region;
use std::error::Error;

use super::utils;

pub async fn build_client() -> Client {
  let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-west-2"));

  let shared_config = aws_config::from_env().region(region_provider).load().await;

  Client::from_conf(Config::from(&shared_config))
}

pub async fn send_email(
  client: Client,
  from_address: &str,
  to_address: &str,
  subject: &str,
  html_content: &str,
) {
  let dest = Destination::builder().to_addresses(to_address).build();
  let subject_content = Content::builder().data(subject).charset("UTF-8").build();
  let body_content = Content::builder()
    .data(html_content)
    .charset("UTF-8")
    .build();
  let body = Body::builder().html(body_content).build();

  let msg = Message::builder()
    .subject(subject_content)
    .body(body)
    .build();

  let email_content = EmailContent::builder().simple(msg).build();

  match client
    .send_email()
    .from_email_address(from_address)
    .destination(dest)
    .content(email_content)
    .send()
    .await
  {
    Ok(_) => {}
    Err(e) => utils::log(utils::Event {
      msg: e.to_string(),
      source: e.source().map(|x| x.to_string()),
      severity: utils::SeverityKind::Error,
    }),
  };
}
