#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate url;
extern crate hyper;

use crate::backend::probe::miner_probe::probe;
use crate::backend::api::miner::Miner;
use url::Host;

pub mod backend;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let res = probe(Host::parse("192.168.1.95").unwrap());
    println!("{:#?}", res.hash_rate());
    rocket::ignite().mount("/", routes![index]).launch();
}