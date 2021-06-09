use super::mail_handlers;
use super::utils;
use super::Db;
use super::SERVICE_NAME;
use mail_service_api::response::MailError;
use std::collections::HashMap;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::Filter;

/// The function that will show all ones to call
pub fn api(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
  api_info()
    .or(mail_new(db.clone()))
    .or(mail_view(db.clone()))
    .recover(handle_rejection)
}

fn api_info() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  let mut info = HashMap::new();
  info.insert("version", "0.1");
  info.insert("name", SERVICE_NAME);
  warp::path!("info").map(move || warp::reply::json(&info))
}

// lets you pass in an arbitrary parameter
fn with<T: Clone + Send>(t: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
  warp::any().map(move || t.clone())
}

fn mail_new(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("mail" / "new")
    .and(with(db))
    .and(warp::body::json())
    .and_then(async move |db, props| mail_handlers::mail_new(db, props).await.map_err(mail_error))
    .map(|x| warp::reply::json(&Some(x).ok_or(())))
}

fn mail_view(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("mail" / "view")
    .and(with(db))
    .and(warp::body::json())
    .and_then(async move |db, props| {
      mail_handlers::mail_view(db, props)
        .await
        .map_err(mail_error)
    })
    .map(|x| warp::reply::json(&Some(x).ok_or(())))
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
  let code;
  let message;

  if err.is_not_found() {
    code = StatusCode::NOT_FOUND;
    message = MailError::NotFound;
  } else if err
    .find::<warp::filters::body::BodyDeserializeError>()
    .is_some()
  {
    code = StatusCode::BAD_REQUEST;
    message = MailError::BadRequest;
  } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
    code = StatusCode::METHOD_NOT_ALLOWED;
    message = MailError::MethodNotAllowed;
  } else if let Some(MailErrorRejection(mail_error)) = err.find() {
    code = StatusCode::BAD_REQUEST;
    message = mail_error.clone();
  } else {
    // We should have expected this... Just log and say its a 500
    utils::log(utils::Event {
      msg: "unknown error kind".to_owned(),
      source: None,
      severity: utils::SeverityKind::Error,
    });
    code = StatusCode::INTERNAL_SERVER_ERROR;
    message = MailError::Unknown;
  }

  Ok(warp::reply::with_status(
    warp::reply::json(&Err::<(), MailError>(message)),
    code,
  ))
}

// This type represents errors that we can generate
// These will be automatically converted to a proper string later
#[derive(Debug)]
pub struct MailErrorRejection(pub MailError);
impl warp::reject::Reject for MailErrorRejection {}

fn mail_error(mail_error: MailError) -> warp::reject::Rejection {
  warp::reject::custom(MailErrorRejection(mail_error))
}
