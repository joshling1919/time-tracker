use super::schema::sessions;
use chrono::DateTime;

#[derive(Queryable)]
#[table_name = "sessions"]
pub struct Session {
    id: i32,
    details: String,
    started_at: DateTime,
    finished_at: DateTime,
}
