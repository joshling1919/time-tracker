use super::schema::sessions;
use chrono::DateTime;

#[derive(Queryable)]
#[table_name = "sessions"]
pub struct Session {
    pub id: i32,
    pub details: String,
    pub started_at: DateTime,
    pub finished_at: DateTime,
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession<'a> {
    pub details: &'a str,
}
