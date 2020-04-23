use connection;
use resources::sessions;
extern crate rocket;

mod router {
    pub fn create_routes() {
        rocket::ignite()
            .manage(connection::init_pool())
            .mount("/sessions", [resources::sessions::handler::post])
    }
}
