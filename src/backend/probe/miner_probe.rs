use failure::{Error, Fail};
use reqwest::header::SET_COOKIE;
use reqwest::Response;
use url::Host;

use crate::backend::api::miner::Miner;
use crate::backend::probe::probe_extractor::AntS9;
use crate::backend::probe::probe_result::AntS9Probe;

// Custom error for authentication
#[derive(Fail, Debug)]
enum ProbeError {
    #[fail(display = "invalid credentials for {}: username: {}, password: {}", host, username, password)]
    InvalidCredentials {
        host: String,
        username: String,
        password: String,
    },
    #[fail(display = "invalid cookie")]
    InvalidCookie,
    #[fail(display = "unreachable host: {}", host)]
    UnreachableHost {
        host: String,
        err: reqwest::Error,
    },
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
    // Send API request to miner
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
    // Extract cookie from response
    let cookie = extract_cookie(req, host, user, password)?;
    Ok(cookie)
}

// Extract cookie from response
fn extract_cookie(req: reqwest::RequestBuilder, host: &Host, user: &str, password: &str) -> Result<String, ProbeError> {
    let response: Result<Response, reqwest::Error> = req.send();
    let result = response
        .or_else(|e: reqwest::Error| {
            Err(ProbeError::UnreachableHost { host: host.to_string(), err: e })
        })
        .and_then(|res: Response| {
            res.headers().get(SET_COOKIE)
                .ok_or(ProbeError::InvalidCredentials { host: host.to_string(), username: user.to_string(), password: password.to_string() })
                .and_then(|header_value| {
                    // Get cookie from header
                    let cookies: String = header_value.to_str().unwrap().to_string();
                    // Strip off unnecessary info
                    let split: Vec<&str> = cookies.split(";").collect();
                    let cookie = split.get(0);
                    // If there is no first array element the cookie string must have been malformed
                    cookie
                        .ok_or(ProbeError::InvalidCookie)
                        .and_then(|first| Ok(first.parse().unwrap()))
                })
        });
    result
}
