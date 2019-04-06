use url::Host;
//use rocket::local::{Client, LocalResponse};
//use rocket::Rocket;
//use rocket::http::{ContentType, Cookie, Status};
use reqwest::header::SET_COOKIE;
use std::{fmt, error};
use crate::backend::api::miner::Miner;
use crate::backend::probe::probe_result::AntS9Probe;
use crate::backend::probe::probe_extractor::AntS9;

#[derive(Debug, Clone)]
struct AuthError;

#[derive(Debug, Clone)]
struct APIError;

// Display method for auth error
impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Authentication error with miner")
    }
}

// Display method for API error
impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "API error with miner")
    }
}

// Auth error wrapper
impl error::Error for AuthError {
    fn description(&self) -> &str {
        "Authentication error with miner"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

// API error wrapper
impl error::Error for APIError {
    fn description(&self) -> &str {
        "API error with miner"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

// Authenticate against miner
fn authenticate(host: &Host, user: &str, password: &str) -> Result<String, AuthError> {
    // This will POST a body of `foo=bar&baz=quux`
    let params = [("luci_username", user), ("luci_password", password)];
    let client = reqwest::Client::builder()
        .redirect(reqwest::RedirectPolicy::none())
        .build().expect("Failed to build http client");
    let req = client.post(url::Url::parse(format!("http://{}/cgi-bin/luci", host.to_string()).as_str()).expect("Failed to parse AUTH URL"))
        .form(&params).header(reqwest::header::CONTENT_LENGTH, 40);
    let res = req.send().expect("Error with api auth request!");
    match res.headers().get(SET_COOKIE) {
        Some(v) => {
            let cookies: String = v.to_str().unwrap().to_string();
            let split: Vec<&str> = cookies.split(";").collect();
            let cookie = split.get(0).expect("Empty vector");
            Ok(cookie.parse().unwrap())
        },
        None => Err(AuthError)
    }
    /*let rocket: Rocket = rocket::ignite();
    let client: Client = Client::new(rocket).expect("valid rocket");
    let form_encoded: ContentType = ContentType::new("application", "x-www-form-urlencoded");
    let response: LocalResponse = client.get("/")
        .header(form_encoded)
        .remote("172.217.17.78:80".parse().unwrap())
//        .remote(format!("{}:80", host.to_string()).parse().unwrap())
        .body(format!("luci_username={}&luci_password={}", user, password))
        .dispatch();
    if response.status() != Status::Found {
        // Error during authentication
        return Err(AuthError);
    }
    let cookie = response.cookies().get(0).unwrap().clone();
    Result::Ok(cookie.into_owned())*/
}

// Probe a miner at the given http endpoint url.
pub fn probe(host: Host) -> impl Miner {
    // Authenticate
    let auth_cookie: String = authenticate(&host, "root", "paplant").expect("Failed to get cookie");
    let http_response = probe_req(&host, &auth_cookie).expect("Failed to get API request");
    let ants9 = AntS9::from(http_response);
    ants9
}

// Make http request to the miner with an authentication cookie
fn probe_req(host: &Host, auth_cookie: &str) -> Result<AntS9Probe, APIError> {
    let client = reqwest::Client::new();
    let req = client.get(url::Url::parse(format!("http://{}/cgi-bin/luci/admin/status/miner/api_status", host.to_string()).as_str()).expect("Failed to parse API URL"))
        .header(reqwest::header::COOKIE, auth_cookie);
    let mut res = req.send().expect("Error with API status request");
    let parsed: AntS9Probe = res.json().expect("Failed to parse JSON from http request");
    Result::Ok(parsed)
}