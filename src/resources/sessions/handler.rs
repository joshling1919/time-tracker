use diesel;
use diesel::prelude::*;
use resources::sessions::Session;
use schema::sessions;

pub fn insert(session: Session, connection: &PgConnection) -> QueryResult<Session> {
    diesel::insert_into(sessions::table)
        .values(&session)
        .get_result(connection)
}
