#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate url;

use crate::backend::probe::miner_probe::probe;
use crate::backend::api::miner::Miner;
use url::Host;
use std::time::{Duration, Instant};

pub mod backend;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let before = Instant::now();
    let res = probe(Host::parse("192.168.1.95").unwrap(), "root", "paplant").expect("Failed to probe");
    let duration: Duration= Instant::now().duration_since(before);
    println!("Time taken: {:?}ms", duration.as_millis());
    println!("{:#?}", res.hash_rate());
    rocket::ignite().mount("/", routes![index]).launch();
}