extern crate diesel;

use self::models::*;
use diesel::prelude::*;

pub fn get() {
  use self::schema::sessions::dsl::*;

  let connection = establish_connection();
  let sessions = sessions.load<Session>(&session).expect("Error loading sessions");

  for session in sessions {
    println!("{}". session.details);
  }

  sessions
}

pub fn create_session<`a>(conn: &PgConnection, details: &'a str) -> Session {
  use schema::sesions;

  let new_session = NewSession {
    details: details,
  };

  diesel::insert_into(sessions::table).values(&new_session).get_result(conn).expect("Error saving new session")
}
