use url::Host;
use reqwest::header::SET_COOKIE;
use crate::backend::api::miner::Miner;
use crate::backend::probe::probe_result::AntS9Probe;
use crate::backend::probe::probe_extractor::AntS9;

use failure::{Error, Fail};

// Custom error for authentication
#[derive(Fail, Debug)]
enum ProbeError {
    #[fail(display = "invalid credentials: {}, {}", username, password)]
    InvalidCredentials {
        username: String,
        password: String
    },
    #[fail(display = "invalid cookie")]
    InvalidCookie
}

// Probe a miner at the given http endpoint url.
pub fn probe(host: Host, user: &str, password: &str) -> Result<impl Miner, Error> {
    // Authenticate
    let auth_cookie: String = authenticate(&host, user, password)?;
    // Make API request for status
    let http_response = probe_req(&host, &auth_cookie)?;
    // Get useful information and return it
    Ok(AntS9::from(http_response))
}

// Make http request to the miner with an authentication cookie
fn probe_req(host: &Host, auth_cookie: &str) -> Result<AntS9Probe, Error> {
    let client = reqwest::Client::new();
    let req = client.get(url::Url::parse(format!("http://{}/cgi-bin/luci/admin/status/miner/api_status", host.to_string()).as_str())?)
        .header(reqwest::header::COOKIE, auth_cookie);
    let mut res = req.send()?;
    // Parse JSON response into ANTS9Probe struct
    let parsed: AntS9Probe = res.json()?;
    Result::Ok(parsed)
}

// Authenticate against miner
fn authenticate(host: &Host, user: &str, password: &str) -> Result<String, Error> {
    // braiins OS username password credentials
    let params = [("luci_username", user), ("luci_password", password)];
    let client = reqwest::Client::builder()
        .redirect(reqwest::RedirectPolicy::none())
        .build()?;
    let req = client.post(url::Url::parse(format!("http://{}/cgi-bin/luci", host.to_string()).as_str())?)
        .form(&params).header(reqwest::header::CONTENT_LENGTH, 40);
    let res = req.send()?;
    // Extract cookie from response
    let cookie = extract_cookie(res, user, password)?;
    Ok(cookie)
}

// Extract cookie from response
fn extract_cookie(res: reqwest::Response, user: &str, password: &str) -> Result<String, ProbeError> {
    match res.headers().get(SET_COOKIE) {
        Some(v) => {
            // Get cookie from header
            let cookies: String = v.to_str().unwrap().to_string();
            // Strip off unnecessary info
            let split: Vec<&str> = cookies.split(";").collect();
            let cookie = split.get(0);
            // If there is no first array element the cookie string must have been malformed
            match cookie {
                Some(v) => Ok(v.parse().unwrap()),
                None => Err(ProbeError::InvalidCookie)
            }
        },
        None => Err(ProbeError::InvalidCredentials { username: user.to_string(), password: password.to_string() })
    }
}
