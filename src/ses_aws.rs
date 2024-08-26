use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::Client;
use aws_sdk_sesv2::Error;
use aws_types::region::Region;

pub async fn build_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-west-2"));

    let shared_config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .region(region_provider)
        .load()
        .await;

    Client::new(&shared_config)
}

pub async fn send_email(
    client: Client,
    from_address: &str,
    to_address: &str,
    subject: &str,
    html_content: &str,
) -> Result<(), Error> {
    let dest = Destination::builder().to_addresses(to_address).build();
    let subject_content = Content::builder().data(subject).charset("UTF-8").build()?;

    let body_content = Content::builder()
        .data(html_content)
        .charset("UTF-8")
        .build()?;

    let body = Body::builder().html(body_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    client
        .send_email()
        .from_email_address(from_address)
        .destination(dest)
        .content(email_content)
        .send()
        .await?;

    Ok(())
}
