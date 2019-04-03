#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate url;

use crate::backend::probe::miner_probe::probe;
use crate::backend::api::miner::Miner;

pub mod backend;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let res = probe(url::Url::parse("http://1.1.1.1").unwrap());
    println!("{:#?}", res.hash_rate());
    rocket::ignite().mount("/", routes![index]).launch();
}