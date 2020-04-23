#![feature(proc_macro_hygiene, decl_macro)]

extern crate dotenv;

use dotenv::dotenv;

mod router;

fn main() {
    dotenv().ok();
    router::create_routes();
}
