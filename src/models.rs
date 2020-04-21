use super::schema::sessions;
use chrono::DateTime;

#[derive(Queryable)]
#[table_name = "sessions"]
pub struct Session {
    pub details: String,
    pub started_at: DateTime,
    pub finished_at: DateTime,
}
