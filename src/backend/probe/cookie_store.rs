use std::collections::HashMap;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Data, Response};

pub struct CookieStore {
    // Map to store the cookies in according to miner ID.
    cookie_map: HashMap<i32, String>,
}

impl CookieStore {
    // Add a cookie to the store
    pub fn put_cookie(&mut self, key: i32, cookie: String) {
        self.cookie_map.insert(key, cookie);
    }

    // Get a cookie from the map as an option
    pub fn get_cookie(&self, key: i32) -> Option<&String> {
        self.cookie_map.get(&key)
    }

    // Constructor
    pub fn new() -> CookieStore {
        CookieStore { cookie_map: HashMap::new() }
    }

    // Print
    pub fn print(&self) {
        println!("{:?}", self.cookie_map);
    }
}
