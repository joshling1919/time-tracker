use super::schema::sessions;

#[derive(Queryable)]
pub struct Session {
    pub id: i32,
    pub details: String,
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession<'a> {
    pub details: &'a str,
}
