#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate url;

use std::time::{Duration, Instant};

use diesel::SqliteConnection;
use rocket::State;
use url::Host;

use backend::api::miner::MinerStats;
use backend::probe::cookie_store::CookieStore;
use backend::probe::miner_probe::probe;

pub mod backend;
pub mod schema;

#[database("sqlite_db")]
pub struct DbConn(SqliteConnection);

// Cookie store managed in rocket
pub struct CookieStoreState(CookieStore);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let cookie_store = CookieStoreState(CookieStore::new());
    let before = Instant::now();
    let res =
        probe(&CookieStore::new(), Host::parse("192.168.1.95").unwrap(), "root", "paplant").expect("Failed to probe");
    let duration: Duration = Instant::now().duration_since(before);
    println!("Time taken: {:?}ms", duration.as_millis());
    println!("{:#?}", res.hash_rate());
    rocket::ignite()
        .attach(DbConn::fairing())
        .manage(cookie_store)
        .mount("/", routes![index])
        .mount(
            "/miner",
            routes![
                backend::api::miner::create,
                backend::api::miner::read,
                backend::api::miner::update,
                backend::api::miner::delete
            ],
        )
        .launch();
}
