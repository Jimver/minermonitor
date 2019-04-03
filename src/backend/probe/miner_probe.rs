use crate::backend::api::miner::Miner;
use url::Url;
use crate::backend::probe::probe_result::deserialize_reader;
use std::path::Path;
use std::fs::File;
use crate::backend::probe::probe_extractor::AntS9;

// Probe a miner at the given http endpoint url.
pub fn probe(http_endpoint: Url) -> impl Miner {
    // Return a result from a pre made json file for now.
    let json_file_path = Path::new("resources/miner_probe_example.json");
    let json_file = File::open(json_file_path).expect("file not found");
    let deserialized = deserialize_reader(json_file).expect("error while reading json");
    let ants9 = AntS9::from(deserialized);
    ants9

    // TODO actual http request with authentication
}