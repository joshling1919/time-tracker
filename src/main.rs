#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod handler;
use self::handler::get;

#[get("/sessions")]
fn index() -> String {
    get();
    format!("hello world")
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
