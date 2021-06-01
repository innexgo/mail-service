use log_service_api::request::LogSeverityKind;

#[derive(Clone, Debug)]
pub struct Event {
  pub event_id: i64,
  pub creation_time: i64,
  pub session_id: i64,
  pub msg: String,
  pub severity_kind: LogSeverityKind,
}

#[derive(Clone, Debug)]
pub struct Session {
  pub session_id: i64,
  pub creation_time: i64,
  pub service_name: String,
}
