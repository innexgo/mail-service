use super::log_db_types::*;
use super::utils::current_time_millis;
use rusqlite::{named_params, params, Savepoint, Connection, OptionalExtension};
use std::convert::{TryFrom, TryInto};

// returns the max event id and adds 1 to it
fn next_id(con: &Connection) -> Result<i64, rusqlite::Error> {
  let sql = "SELECT max(event_id) FROM event";
  con.query_row(sql, [], |row| row.get(0))
}

impl TryFrom<&rusqlite::Row<'_>> for Event {
  type Error = rusqlite::Error;

  // select * from event order only, otherwise it will fail
  fn try_from(row: &rusqlite::Row) -> Result<Event, rusqlite::Error> {
    Ok(Event {
      event_id: row.get(0)?,
      creation_time: row.get(1)?,
      creator_user_id: row.get(2)?,
      event_hash: row.get(3)?,
      // means that there's a mismatch between the values of the enum and the value stored in the column
      event_kind: row
        .get::<_, u8>(4)?
        .try_into()
        .map_err(|x| rusqlite::Error::IntegralValueOutOfRange(4, x as i64))?,
      duration: row.get(5)?,
    })
  }
}

pub fn add(
  con: &mut Savepoint,
  creator_user_id: i64,
  event_hash: String,
  event_kind: log_service_api::request::EventKind,
  duration: i64,
) -> Result<Event, rusqlite::Error> {
  let sp = con.savepoint()?;
  let event_id = next_id(&sp)?;
  let creation_time = current_time_millis();

  let sql = "INSERT INTO event values (?, ?, ?, ?, ?, ?)";
  sp.execute(
    sql,
    params![
      event_id,
      creation_time,
      creator_user_id,
      event_hash,
      event_kind.clone() as u8,
      duration,
    ],
  )?;

  // commit savepoint
  sp.commit()?;

  // return event
  Ok(Event {
    event_id,
    creation_time,
    creator_user_id,
    event_hash,
    event_kind,
    duration,
  })
}

pub fn get_by_event_id(
  con: &Connection,
  event_id: i64,
) -> Result<Option<Event>, rusqlite::Error> {
  let sql = "SELECT * FROM event WHERE event_id=?";
  con
    .query_row(sql, params![event_id], |row| row.try_into())
    .optional()
}

pub fn get_by_event_hash(
  con: &Connection,
  event_hash: &str,
) -> Result<Option<Event>, rusqlite::Error> {
  let sql = "SELECT * FROM event WHERE event_hash=? ORDER BY event_id DESC LIMIT 1";
  con
    .query_row(sql, params![event_hash], |row| row.try_into())
    .optional()
}

pub fn query(
  con: &Connection,
  props: log_service_api::request::EventViewProps
) -> Result<Vec<Event>, rusqlite::Error> {
  // TODO prevent getting meaningless duration

  let sql = [
    "SELECT a.* FROM event a",
    if props.only_recent {
        " INNER JOIN (SELECT max(event_id) id FROM event GROUP BY event_hash) maxids ON maxids.id = a.event_id"
    } else {
        ""
    },
    " WHERE 1 = 1",
    " AND (:event_id      == NULL OR a.event_id = :event_id)",
    " AND (:creation_time   == NULL OR a.creation_time = :creation_time)",
    " AND (:creation_time   == NULL OR a.creation_time > :min_creation_time)",
    " AND (:creation_time   == NULL OR a.creation_time > :max_creation_time)",
    " AND (:creator_user_id == NULL OR a.creator_user_id = :creator_user_id)",
    " AND (:duration        == NULL OR a.duration = :duration)",
    " AND (:duration        == NULL OR a.duration > :min_duration)",
    " AND (:duration        == NULL OR a.duration > :max_duration)",
    " AND (:event_kind    == NULL OR a.event_kind = :event_kind)",
    " ORDER BY u.event_id",
    " LIMIT :offset, :count",
  ]
  .join("");

  let mut stmnt = con.prepare(&sql)?;

  let results = stmnt
    .query(named_params! {
        "event_id": props.event_id,
        "creator_user_id": props.creator_user_id,
        "creation_time": props.creation_time,
        "min_creation_time": props.min_creation_time,
        "max_creation_time": props.max_creation_time,
        "duration": props.duration,
        "min_duration": props.min_duration,
        "max_duration": props.max_duration,
        "event_kind": props.event_kind.map(|x| x as u8),
        "offset": props.offset,
        "count": props.offset,
    })?
    .and_then(|row| row.try_into())
    .filter_map(|x: Result<Event, rusqlite::Error>| x.ok());
  Ok(results.collect::<Vec<Event>>())
}
