use log_service_api::request::LogSeverityKind;

#[derive(Clone, Debug)]
pub struct Event {
    pub event_id:i64,
    pub creation_time: i64,
    pub source:String,
    pub severity_kind: LogSeverityKind,
    pub msg: String
}

pub struct Heartbeat {
    pub heartbeat_id:i64,
    pub creation_time: i64,
    pub source:String
}
