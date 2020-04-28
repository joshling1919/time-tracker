use crate::connection::*;
use crate::models::*;
use crate::schema::sessions::dsl::*;
use diesel::prelude::*;

pub fn get() {
    let connection: PgConnection = establish_connection();
    let sessions = sessions
        .load::<Session>(&connection)
        .expect("Error loading sessions");

    for session in sessions {
        println!("{}".session.details);
    }

    sessions
}

// pub fn create_session<`a>(conn: &PgConnection, details: &'a str) -> Session {
//   use schema::sesions;

//   let new_session = NewSession {
//     details: details,
//   };

//   diesel::insert_into(sessions::table).values(&new_session).get_result(conn).expect("Error saving new session")
// }
