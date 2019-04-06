use url::Host;
//use rocket::local::{Client, LocalResponse};
//use rocket::Rocket;
//use rocket::http::{ContentType, Cookie, Status};
use reqwest::header::SET_COOKIE;
use std::{fmt, error};
use crate::backend::api::miner::Miner;
use crate::backend::probe::probe_result::{deserialize_string, deserialize_reader, AntS9Probe};
use crate::backend::probe::probe_extractor::AntS9;
use std::error::Error;
use std::io::Read;
use url::form_urlencoded::parse;

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
    let mut res = req.send().expect("Error with api auth request!");
    match res.headers().get(SET_COOKIE) {
        Some(v) => {
            let cookies: String = v.to_str().unwrap().to_string();
            let split: Vec<_> = cookies.split(";").collect();
            let cookie = split[0];
            println!("{:?}", cookie);
            Ok(cookies)
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
    let custom_string = r#"{
        "stats": [
            {
                "STATUS": [
                {
                    "STATUS": "S",
                    "When": 1553530005,
                    "Code": 70,
                    "Msg": "BMMiner stats",
                    "Description": "bmminer bOS_am1-s9-20190221-0_572dd48c"
                }
            ],
                "STATS": [
                {
                    "BMMiner": "bOS_am1-s9-20190221-0_572dd48c",
                    "Miner": "bOS_am1-s9-20190221-0_572dd48c",
                    "CompileTime": "",
                    "Type": "braiins-am1-s9"
                },
                {
                    "STATS": 1,
                    "ID": "BC50",
                    "Elapsed": 89509,
                    "Calls": 0,
                    "Wait": 0,
                    "Max": 0,
                    "Min": 99999999,
                    "GHS 5s": "13815.68",
                    "GHS av": 13838.41,
                    "miner_count": 3,
                    "frequency": "681",
                    "fan_num": 2,
                    "fan1": 0,
                    "fan2": 0,
                    "fan3": 0,
                    "fan4": 0,
                    "fan5": 3360,
                    "fan6": 4440,
                    "fan7": 0,
                    "fan8": 0,
                    "voltage1": 0,
                    "voltage2": 0,
                    "voltage3": 0,
                    "voltage4": 0,
                    "voltage5": 0,
                    "voltage6": 8.8,
                    "voltage7": 8.7,
                    "voltage8": 8.8,
                    "voltage9": 0,
                    "voltage10": 0,
                    "voltage11": 0,
                    "voltage12": 0,
                    "voltage13": 0,
                    "voltage14": 0,
                    "voltage15": 0,
                    "voltage16": 0,
                    "temp_num": 3,
                    "temp1": 0,
                    "temp2": 0,
                    "temp3": 0,
                    "temp4": 0,
                    "temp5": 0,
                    "temp6": 55,
                    "temp7": 54,
                    "temp8": 52,
                    "temp9": 0,
                    "temp10": 0,
                    "temp11": 0,
                    "temp12": 0,
                    "temp13": 0,
                    "temp14": 0,
                    "temp15": 0,
                    "temp16": 0,
                    "temp2_1": 0,
                    "temp2_2": 0,
                    "temp2_3": 0,
                    "temp2_4": 0,
                    "temp2_5": 0,
                    "temp2_6": 74,
                    "temp2_7": 75,
                    "temp2_8": 73,
                    "temp2_9": 0,
                    "temp2_10": 0,
                    "temp2_11": 0,
                    "temp2_12": 0,
                    "temp2_13": 0,
                    "temp2_14": 0,
                    "temp2_15": 0,
                    "temp2_16": 0,
                    "temp3_1": 0,
                    "temp3_2": 0,
                    "temp3_3": 0,
                    "temp3_4": 0,
                    "temp3_5": 0,
                    "temp3_6": 0,
                    "temp3_7": 0,
                    "temp3_8": 0,
                    "temp3_9": 0,
                    "temp3_10": 0,
                    "temp3_11": 0,
                    "temp3_12": 0,
                    "temp3_13": 0,
                    "temp3_14": 0,
                    "temp3_15": 0,
                    "temp3_16": 0,
                    "freq_desc6": "factory set frequency 637 MHz",
                    "freq_desc7": "factory set frequency 681 MHz",
                    "freq_desc8": "factory set frequency 637 MHz",
                    "freq_avg1": 0,
                    "freq_avg2": 0,
                    "freq_avg3": 0,
                    "freq_avg4": 0,
                    "freq_avg5": 0,
                    "freq_avg6": 637,
                    "freq_avg7": 681,
                    "freq_avg8": 637,
                    "freq_avg9": 0,
                    "freq_avg10": 0,
                    "freq_avg11": 0,
                    "freq_avg12": 0,
                    "freq_avg13": 0,
                    "freq_avg14": 0,
                    "freq_avg15": 0,
                    "freq_avg16": 0,
                    "total_rateideal": 14002.76,
                    "total_freqavg": 651.66,
                    "total_acn": 189,
                    "total_rate": 13815.68,
                    "chain_rateideal1": 0,
                    "chain_rateideal2": 0,
                    "chain_rateideal3": 0,
                    "chain_rateideal4": 0,
                    "chain_rateideal5": 0,
                    "chain_rateideal6": 4573.66,
                    "chain_rateideal7": 4854.16,
                    "chain_rateideal8": 4574.93,
                    "chain_rateideal9": 0,
                    "chain_rateideal10": 0,
                    "chain_rateideal11": 0,
                    "chain_rateideal12": 0,
                    "chain_rateideal13": 0,
                    "chain_rateideal14": 0,
                    "chain_rateideal15": 0,
                    "chain_rateideal16": 0,
                    "temp_max": 0,
                    "Device Hardware%": 0.0003,
                    "no_matching_work": 827,
                    "chain_acn1": 0,
                    "chain_acn2": 0,
                    "chain_acn3": 0,
                    "chain_acn4": 0,
                    "chain_acn5": 0,
                    "chain_acn6": 63,
                    "chain_acn7": 63,
                    "chain_acn8": 63,
                    "chain_acn9": 0,
                    "chain_acn10": 0,
                    "chain_acn11": 0,
                    "chain_acn12": 0,
                    "chain_acn13": 0,
                    "chain_acn14": 0,
                    "chain_acn15": 0,
                    "chain_acn16": 0,
                    "chain_cores6": 7180,
                    "chain_cores7": 7128,
                    "chain_cores8": 7182,
                    "chain_acs1": "",
                    "chain_acs2": "",
                    "chain_acs3": "",
                    "chain_acs4": "",
                    "chain_acs5": "",
                    "chain_acs6": " oooooooo oooooooo oooooooo oooooooo oooooooo oooooooo oooooooo ooooooo",
                    "chain_acs7": " oooooooo oooooooo oooooooo oooooooo oooooooo oooooooo oooooooo ooooooo",
                    "chain_acs8": " oooooooo oooooooo oooooooo oooooooo oooooooo oooooooo oooooooo ooooooo",
                    "chain_acs9": "",
                    "chain_acs10": "",
                    "chain_acs11": "",
                    "chain_acs12": "",
                    "chain_acs13": "",
                    "chain_acs14": "",
                    "chain_acs15": "",
                    "chain_acs16": "",
                    "chain_hw1": 0,
                    "chain_hw2": 0,
                    "chain_hw3": 0,
                    "chain_hw4": 0,
                    "chain_hw5": 0,
                    "chain_hw6": 56,
                    "chain_hw7": 694,
                    "chain_hw8": 76,
                    "chain_hw9": 0,
                    "chain_hw10": 0,
                    "chain_hw11": 0,
                    "chain_hw12": 0,
                    "chain_hw13": 0,
                    "chain_hw14": 0,
                    "chain_hw15": 0,
                    "chain_hw16": 0,
                    "chain_hwrate1": 0,
                    "chain_hwrate2": 0,
                    "chain_hwrate3": 0,
                    "chain_hwrate4": 0,
                    "chain_hwrate5": 0,
                    "chain_hwrate6": 2.538394,
                    "chain_hwrate7": 38.691123,
                    "chain_hwrate8": 1.653272,
                    "chain_hwrate9": 0,
                    "chain_hwrate10": 0,
                    "chain_hwrate11": 0,
                    "chain_hwrate12": 0,
                    "chain_hwrate13": 0,
                    "chain_hwrate14": 0,
                    "chain_hwrate15": 0,
                    "chain_hwrate16": 0,
                    "chain_rate1": "",
                    "chain_rate2": "",
                    "chain_rate3": "",
                    "chain_rate4": "",
                    "chain_rate5": "",
                    "chain_rate6": "4466.53",
                    "chain_rate7": "4760.62",
                    "chain_rate8": "4588.53",
                    "chain_rate9": "",
                    "chain_rate10": "",
                    "chain_rate11": "",
                    "chain_rate12": "",
                    "chain_rate13": "",
                    "chain_rate14": "",
                    "chain_rate15": "",
                    "chain_rate16": "",
                    "chain_xtime6": "{X42=1,X46=15}",
                    "chain_xtime7": "{X13=1,X29=168}",
                    "chain_xtime8": "{}",
                    "chain_offside_6": "0",
                    "chain_offside_7": "0",
                    "chain_offside_8": "0",
                    "chain_opencore_6": "0",
                    "chain_opencore_7": "0",
                    "chain_opencore_8": "0",
                    "miner_version": "26.0.1.3"
                }
            ],
                "id": 1
            }
        ],
        "pools": [
            {
                "STATUS": [
                {
                    "STATUS": "S",
                    "When": 1553530005,
                    "Code": 7,
                    "Msg": "1 Pool(s)",
                    "Description": "bmminer bOS_am1-s9-20190221-0_572dd48c"
                }
            ],
                "POOLS": [
                {
                    "POOL": 0,
                    "URL": "stratum+tcp://stratum.slushpool.com:3333",
                    "Status": "Alive",
                    "Priority": 0,
                    "Quota": 1,
                    "Long Poll": "N",
                    "Getworks": 3203,
                    "Accepted": 29452,
                    "Rejected": 229,
                    "Discarded": 48086,
                    "Stale": 0,
                    "Get Failures": 0,
                    "Remote Failures": 0,
                    "User": "cowchaser.ANTS9-5",
                    "Last Share Time": "0:00:05",
                    "Diff": "9.64K",
                    "LastDiff": 9641,
                    "Diff1 Shares": 0,
                    "Proxy Type": "",
                    "Proxy": "",
                    "Difficulty Accepted": 286142783,
                    "Difficulty Rejected": 2228648,
                    "Difficulty Stale": 0,
                    "Last Share Difficulty": 9641,
                    "Has Stratum": true,
                    "Asic Boost": true,
                    "Stratum Active": true,
                    "Stratum URL": "stratum.slushpool.com",
                    "Has GBT": false,
                    "Best Share": 122319835,
                    "Pool Rejected%": 0.7728,
                    "Pool Stale%": 0
                }
            ],
                "id": 1
            }
        ],
        "summary": [
            {
                "STATUS": [
                {
                    "STATUS": "S",
                    "When": 1553530005,
                    "Code": 11,
                    "Msg": "Summary",
                    "Description": "bmminer bOS_am1-s9-20190221-0_572dd48c"
                }
            ],
                "SUMMARY": [
                {
                    "Elapsed": 89509,
                    "GHS 5s": "13815.68",
                    "GHS av": 13838.41,
                    "Hashrate1m": 14101.233981,
                    "Hashrate15m": 13853.16519,
                    "Hashrate24h": 13837.988845,
                    "Found Blocks": 0,
                    "Getworks": 3203,
                    "Accepted": 29452,
                    "Rejected": 229,
                    "Hardware Errors": 827,
                    "Utility": 19.74,
                    "Discarded": 48086,
                    "Stale": 0,
                    "Get Failures": 0,
                    "Local Work": 4558972,
                    "Remote Failures": 0,
                    "Network Blocks": 144,
                    "Total MH": 1238647994691,
                    "Work Utility": 193302.19,
                    "Difficulty Accepted": 286142783,
                    "Difficulty Rejected": 2228648,
                    "Difficulty Stale": 0,
                    "Best Share": 122319835,
                    "Device Hardware%": 0.0003,
                    "Device Rejected%": 0.7728,
                    "Pool Rejected%": 0.7728,
                    "Pool Stale%": 0,
                    "Last getwork": 1553530003
                }
            ],
                "id": 1
            }
        ],
        "devs": [
            {
                "STATUS": [
                {
                    "STATUS": "S",
                    "When": 1553530005,
                    "Code": 9,
                    "Msg": "1 ASC(s)",
                    "Description": "bmminer bOS_am1-s9-20190221-0_572dd48c"
                }
            ],
                "DEVS": [
                {
                    "ASC": 0,
                    "Name": "BC5",
                    "ID": 0,
                    "Enabled": "Y",
                    "Status": "Alive",
                    "TempAVG": 0,
                    "TempMAX": 0,
                    "TempMIN": 0,
                    "CHIP": 63,
                    "FREQ": 637,
                    "DUTY": 4440,
                    "MHS av": 4466530,
                    "MHS 5s": 4466530,
                    "MHS 1m": 4466530,
                    "MHS 5m": 4466530,
                    "MHS 15m": 4466530,
                    "nominal MHS": 4573660,
                    "maximal MHS": 4573660,
                    "Accepted": 9554,
                    "Rejected": 84,
                    "Hardware Errors": 56
                },
                {
                    "ASC": 1,
                    "Name": "BC5",
                    "ID": 1,
                    "Enabled": "Y",
                    "Status": "Alive",
                    "TempAVG": 0,
                    "TempMAX": 0,
                    "TempMIN": 0,
                    "CHIP": 63,
                    "FREQ": 681,
                    "DUTY": 0,
                    "MHS av": 4760620,
                    "MHS 5s": 4760620,
                    "MHS 1m": 4760620,
                    "MHS 5m": 4760620,
                    "MHS 15m": 4760620,
                    "nominal MHS": 4854160,
                    "maximal MHS": 4854160,
                    "Accepted": 10251,
                    "Rejected": 74,
                    "Hardware Errors": 694
                },
                {
                    "ASC": 2,
                    "Name": "BC5",
                    "ID": 2,
                    "Enabled": "Y",
                    "Status": "Alive",
                    "TempAVG": 0,
                    "TempMAX": 0,
                    "TempMIN": 0,
                    "CHIP": 63,
                    "FREQ": 637,
                    "DUTY": 0,
                    "MHS av": 4588530,
                    "MHS 5s": 4588530,
                    "MHS 1m": 4588530,
                    "MHS 5m": 4588530,
                    "MHS 15m": 4588530,
                    "nominal MHS": 4574930,
                    "maximal MHS": 4574930,
                    "Accepted": 9647,
                    "Rejected": 70,
                    "Hardware Errors": 76
                }
            ],
                "id": 1
            }
        ],
        "fanctrl": [
            {
                "STATUS": [
                {
                    "STATUS": "S",
                    "When": 1553530005,
                    "Code": 127,
                    "Msg": "Fan control values",
                    "Description": "bmminer bOS_am1-s9-20190221-0_572dd48c"
                }
            ],
                "FANCTRL": [
                {
                    "Mode": "automatic",
                    "TargetTemp": 75,
                    "TargetPwm": 100,
                    "Temperature": 75,
                    "Output": 42,
                    "Interval": 7.011423
                }
            ],
                "id": 1
            }
        ],
        "id": 1
    }
    "#;

//    println!("{:?}", http_response);
    // Return a result from a pre made json file for now.
//    let json_file_path = Path::new("resources/miner_probe_example.json");
//    let json_file = File::open(json_file_path).expect("file not found");
    let ants9 = AntS9::from(http_response);
    ants9
}

// Make http request to the miner with an authentication cookie
fn probe_req(host: &Host, auth_cookie: &str) -> Result<AntS9Probe, APIError> {
    /*let rocket: Rocket = rocket::ignite();
    let client: Client = Client::new(rocket).expect("valid rocket");
    let mut response: LocalResponse = client.get("/cgi-bin/luci/admin/status/miner/api_status")
        .header(ContentType::JSON)
        .remote(host.to_string().parse().unwrap())
        .cookie(auth_cookie.clone())
        .dispatch();
    if response.status() != Status::Ok {
        // Error during authentication
        return Err(APIError)
    }
    Result::Ok(response.body_string().unwrap())*/
    let client = reqwest::Client::new();
    let req = client.get(url::Url::parse(format!("http://{}/cgi-bin/luci/admin/status/miner/api_status", host.to_string()).as_str()).expect("Failed to parse API URL"))
        .header(reqwest::header::COOKIE, auth_cookie);
    let mut res = req.send().expect("Error with API status request");
    let parsed: AntS9Probe = res.json().expect("Failed to parse JSON from http request");
    Result::Ok(parsed)
}