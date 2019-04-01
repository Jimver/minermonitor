/*
Result of probing a miner
*/
use serde::{Serialize, Deserialize};

pub fn deserialize(string: String) {
    let deserialized: Devs = serde_json::from_str(&string).unwrap();
}

#[derive(Serialize, Deserialize)]
struct Devs {
    #[serde(rename = "STATUS")]
    _status: Vec<Status>,
    #[serde(rename = "DEVS")]
    _devs: Vec<Devs1>,
    id: i64,
}

#[derive(Serialize, Deserialize)]
struct Devs1 {
    #[serde(rename = "ASC")]
    _asc: i64,
    #[serde(rename = "Name")]
    _name: String,
    #[serde(rename = "ID")]
    _id: i64,
    #[serde(rename = "Enabled")]
    _enabled: String,
    #[serde(rename = "Status")]
    _status: String,
    #[serde(rename = "TempAVG")]
    temp_avg: i64,
    #[serde(rename = "TempMAX")]
    temp_max: i64,
    #[serde(rename = "TempMIN")]
    temp_min: i64,
    #[serde(rename = "CHIP")]
    _chip: i64,
    #[serde(rename = "FREQ")]
    _freq: i64,
    #[serde(rename = "DUTY")]
    _duty: i64,
    #[serde(rename = "MHS av")]
    mhs_av: i64,
    #[serde(rename = "MHS 5s")]
    mhs_5_s: i64,
    #[serde(rename = "MHS 1m")]
    mhs_1_m: i64,
    #[serde(rename = "MHS 5m")]
    mhs_5_m: i64,
    #[serde(rename = "MHS 15m")]
    mhs_15_m: i64,
    #[serde(rename = "nominal MHS")]
    nominal_mhs: i64,
    #[serde(rename = "maximal MHS")]
    maximal_mhs: i64,
    #[serde(rename = "Accepted")]
    _accepted: i64,
    #[serde(rename = "Rejected")]
    _rejected: i64,
    #[serde(rename = "Hardware Errors")]
    hardware_errors: i64,
}

#[derive(Serialize, Deserialize)]
struct Fanctrl {
    #[serde(rename = "STATUS")]
    _status: Vec<Status>,
    #[serde(rename = "FANCTRL")]
    _fanctrl: Vec<Fanctrl1>,
    id: i64,
}

#[derive(Serialize, Deserialize)]
struct Fanctrl1 {
    #[serde(rename = "Mode")]
    _mode: String,
    #[serde(rename = "TargetTemp")]
    target_temp: i64,
    #[serde(rename = "TargetPwm")]
    target_pwm: i64,
    #[serde(rename = "Temperature")]
    _temperature: i64,
    #[serde(rename = "Output")]
    _output: i64,
    #[serde(rename = "Interval")]
    _interval: f64,
}

#[derive(Serialize, Deserialize)]
struct Pools {
    #[serde(rename = "STATUS")]
    _status: Vec<Status>,
    #[serde(rename = "POOLS")]
    _pools: Vec<Pools1>,
    id: i64,
}

#[derive(Serialize, Deserialize)]
struct Pools1 {
    #[serde(rename = "POOL")]
    _pool: i64,
    #[serde(rename = "URL")]
    _url: String,
    #[serde(rename = "Status")]
    _status: String,
    #[serde(rename = "Priority")]
    _priority: i64,
    #[serde(rename = "Quota")]
    _quota: i64,
    #[serde(rename = "Long Poll")]
    long_poll: String,
    #[serde(rename = "Getworks")]
    _getworks: i64,
    #[serde(rename = "Accepted")]
    _accepted: i64,
    #[serde(rename = "Rejected")]
    _rejected: i64,
    #[serde(rename = "Discarded")]
    _discarded: i64,
    #[serde(rename = "Stale")]
    _stale: i64,
    #[serde(rename = "Get Failures")]
    get_failures: i64,
    #[serde(rename = "Remote Failures")]
    remote_failures: i64,
    #[serde(rename = "User")]
    _user: String,
    #[serde(rename = "Last Share Time")]
    last_share_time: String,
    #[serde(rename = "Diff")]
    _diff: String,
    #[serde(rename = "LastDiff")]
    last_diff: i64,
    #[serde(rename = "Diff1 Shares")]
    diff_1_shares: i64,
    #[serde(rename = "Proxy Type")]
    proxy_type: String,
    #[serde(rename = "Proxy")]
    _proxy: String,
    #[serde(rename = "Difficulty Accepted")]
    difficulty_accepted: i64,
    #[serde(rename = "Difficulty Rejected")]
    difficulty_rejected: i64,
    #[serde(rename = "Difficulty Stale")]
    difficulty_stale: i64,
    #[serde(rename = "Last Share Difficulty")]
    last_share_difficulty: i64,
    #[serde(rename = "Has Stratum")]
    has_stratum: bool,
    #[serde(rename = "Asic Boost")]
    asic_boost: bool,
    #[serde(rename = "Stratum Active")]
    stratum_active: bool,
    #[serde(rename = "Stratum URL")]
    stratum_url: String,
    #[serde(rename = "Has GBT")]
    has_gbt: bool,
    #[serde(rename = "Best Share")]
    best_share: i64,
    #[serde(rename = "Pool Rejected%")]
    pool_rejected: f64,
    #[serde(rename = "Pool Stale%")]
    pool_stale: i64,
}

#[derive(Serialize, Deserialize)]
struct RootInterface {
    stats: Vec<Stats>,
    pools: Vec<Pools>,
    summary: Vec<Summary>,
    devs: Vec<Devs>,
    fanctrl: Vec<Fanctrl>,
    id: i64,
}

#[derive(Serialize, Deserialize)]
struct Stats {
    #[serde(rename = "STATUS")]
    _status: Vec<Status>,
    #[serde(rename = "STATS")]
    _stats: Vec<Stats1>,
    id: i64,
}

#[derive(Serialize, Deserialize)]
struct Stats1 {
    #[serde(rename = "BMMiner")]
    bm_miner: Option<String>,
    #[serde(rename = "Miner")]
    _miner: Option<String>,
    #[serde(rename = "CompileTime")]
    compile_time: Option<String>,
    #[serde(rename = "Type")]
    _type: Option<String>,
    #[serde(rename = "STATS")]
    _stats: Option<i64>,
    #[serde(rename = "ID")]
    _id: Option<String>,
    #[serde(rename = "Elapsed")]
    _elapsed: Option<i64>,
    #[serde(rename = "Calls")]
    _calls: Option<i64>,
    #[serde(rename = "Wait")]
    _wait: Option<i64>,
    #[serde(rename = "Max")]
    _max: Option<i64>,
    #[serde(rename = "Min")]
    _min: Option<i64>,
    #[serde(rename = "GHS 5s")]
    ghs_5_s: String,
    #[serde(rename = "GHS av")]
    ghs_av: f64,
    miner_count: Option<i64>,
    frequency: Option<String>,
    fan_num: Option<i64>,
    #[serde(rename = "fan1")]
    fan_1: Option<i64>,
    #[serde(rename = "fan2")]
    fan_2: Option<i64>,
    #[serde(rename = "fan3")]
    fan_3: Option<i64>,
    #[serde(rename = "fan4")]
    fan_4: Option<i64>,
    #[serde(rename = "fan5")]
    fan_5: Option<i64>,
    #[serde(rename = "fan6")]
    fan_6: Option<i64>,
    #[serde(rename = "fan7")]
    fan_7: Option<i64>,
    #[serde(rename = "fan8")]
    fan_8: Option<i64>,
    #[serde(rename = "voltage1")]
    voltage_1: Option<i64>,
    #[serde(rename = "voltage2")]
    voltage_2: Option<i64>,
    #[serde(rename = "voltage3")]
    voltage_3: Option<i64>,
    #[serde(rename = "voltage4")]
    voltage_4: Option<i64>,
    #[serde(rename = "voltage5")]
    voltage_5: Option<i64>,
    #[serde(rename = "voltage6")]
    voltage_6: Option<f64>,
    #[serde(rename = "voltage7")]
    voltage_7: Option<f64>,
    #[serde(rename = "voltage8")]
    voltage_8: Option<f64>,
    #[serde(rename = "voltage9")]
    voltage_9: Option<i64>,
    #[serde(rename = "voltage10")]
    voltage_10: Option<i64>,
    #[serde(rename = "voltage11")]
    voltage_11: Option<i64>,
    #[serde(rename = "voltage12")]
    voltage_12: Option<i64>,
    #[serde(rename = "voltage13")]
    voltage_13: Option<i64>,
    #[serde(rename = "voltage14")]
    voltage_14: Option<i64>,
    #[serde(rename = "voltage15")]
    voltage_15: Option<i64>,
    #[serde(rename = "voltage16")]
    voltage_16: Option<i64>,
    temp_num: Option<i64>,
    #[serde(rename = "temp1")]
    temp_1: Option<i64>,
    #[serde(rename = "temp2")]
    temp_2: Option<i64>,
    #[serde(rename = "temp3")]
    temp_3: Option<i64>,
    #[serde(rename = "temp4")]
    temp_4: Option<i64>,
    #[serde(rename = "temp5")]
    temp_5: Option<i64>,
    #[serde(rename = "temp6")]
    temp_6: Option<i64>,
    #[serde(rename = "temp7")]
    temp_7: Option<i64>,
    #[serde(rename = "temp8")]
    temp_8: Option<i64>,
    #[serde(rename = "temp9")]
    temp_9: Option<i64>,
    #[serde(rename = "temp10")]
    temp_10: Option<i64>,
    #[serde(rename = "temp11")]
    temp_11: Option<i64>,
    #[serde(rename = "temp12")]
    temp_12: Option<i64>,
    #[serde(rename = "temp13")]
    temp_13: Option<i64>,
    #[serde(rename = "temp14")]
    temp_14: Option<i64>,
    #[serde(rename = "temp15")]
    temp_15: Option<i64>,
    #[serde(rename = "temp16")]
    temp_16: Option<i64>,
    #[serde(rename = "temp2_1")]
    temp_2_1: Option<i64>,
    #[serde(rename = "temp2_2")]
    temp_2_2: Option<i64>,
    #[serde(rename = "temp2_3")]
    temp_2_3: Option<i64>,
    #[serde(rename = "temp2_4")]
    temp_2_4: Option<i64>,
    #[serde(rename = "temp2_5")]
    temp_2_5: Option<i64>,
    #[serde(rename = "temp2_6")]
    temp_2_6: Option<i64>,
    #[serde(rename = "temp2_7")]
    temp_2_7: Option<i64>,
    #[serde(rename = "temp2_8")]
    temp_2_8: Option<i64>,
    #[serde(rename = "temp2_9")]
    temp_2_9: Option<i64>,
    #[serde(rename = "temp2_10")]
    temp_2_10: Option<i64>,
    #[serde(rename = "temp2_11")]
    temp_2_11: Option<i64>,
    #[serde(rename = "temp2_12")]
    temp_2_12: Option<i64>,
    #[serde(rename = "temp2_13")]
    temp_2_13: Option<i64>,
    #[serde(rename = "temp2_14")]
    temp_2_14: Option<i64>,
    #[serde(rename = "temp2_15")]
    temp_2_15: Option<i64>,
    #[serde(rename = "temp2_16")]
    temp_2_16: Option<i64>,
    #[serde(rename = "temp3_1")]
    temp_3_1: Option<i64>,
    #[serde(rename = "temp3_2")]
    temp_3_2: Option<i64>,
    #[serde(rename = "temp3_3")]
    temp_3_3: Option<i64>,
    #[serde(rename = "temp3_4")]
    temp_3_4: Option<i64>,
    #[serde(rename = "temp3_5")]
    temp_3_5: Option<i64>,
    #[serde(rename = "temp3_6")]
    temp_3_6: Option<i64>,
    #[serde(rename = "temp3_7")]
    temp_3_7: Option<i64>,
    #[serde(rename = "temp3_8")]
    temp_3_8: Option<i64>,
    #[serde(rename = "temp3_9")]
    temp_3_9: Option<i64>,
    #[serde(rename = "temp3_10")]
    temp_3_10: Option<i64>,
    #[serde(rename = "temp3_11")]
    temp_3_11: Option<i64>,
    #[serde(rename = "temp3_12")]
    temp_3_12: Option<i64>,
    #[serde(rename = "temp3_13")]
    temp_3_13: Option<i64>,
    #[serde(rename = "temp3_14")]
    temp_3_14: Option<i64>,
    #[serde(rename = "temp3_15")]
    temp_3_15: Option<i64>,
    #[serde(rename = "temp3_16")]
    temp_3_16: Option<i64>,
    #[serde(rename = "freq_desc6")]
    freq_desc_6: Option<String>,
    #[serde(rename = "freq_desc7")]
    freq_desc_7: Option<String>,
    #[serde(rename = "freq_desc8")]
    freq_desc_8: Option<String>,
    #[serde(rename = "freq_avg1")]
    freq_avg_1: Option<i64>,
    #[serde(rename = "freq_avg2")]
    freq_avg_2: Option<i64>,
    #[serde(rename = "freq_avg3")]
    freq_avg_3: Option<i64>,
    #[serde(rename = "freq_avg4")]
    freq_avg_4: Option<i64>,
    #[serde(rename = "freq_avg5")]
    freq_avg_5: Option<i64>,
    #[serde(rename = "freq_avg6")]
    freq_avg_6: Option<i64>,
    #[serde(rename = "freq_avg7")]
    freq_avg_7: Option<i64>,
    #[serde(rename = "freq_avg8")]
    freq_avg_8: Option<i64>,
    #[serde(rename = "freq_avg9")]
    freq_avg_9: Option<i64>,
    #[serde(rename = "freq_avg10")]
    freq_avg_10: Option<i64>,
    #[serde(rename = "freq_avg11")]
    freq_avg_11: Option<i64>,
    #[serde(rename = "freq_avg12")]
    freq_avg_12: Option<i64>,
    #[serde(rename = "freq_avg13")]
    freq_avg_13: Option<i64>,
    #[serde(rename = "freq_avg14")]
    freq_avg_14: Option<i64>,
    #[serde(rename = "freq_avg15")]
    freq_avg_15: Option<i64>,
    #[serde(rename = "freq_avg16")]
    freq_avg_16: Option<i64>,
    total_rateideal: Option<f64>,
    total_freqavg: Option<f64>,
    total_acn: Option<i64>,
    total_rate: Option<f64>,
    #[serde(rename = "chain_rateideal1")]
    chain_rateideal_1: Option<i64>,
    #[serde(rename = "chain_rateideal2")]
    chain_rateideal_2: Option<i64>,
    #[serde(rename = "chain_rateideal3")]
    chain_rateideal_3: Option<i64>,
    #[serde(rename = "chain_rateideal4")]
    chain_rateideal_4: Option<i64>,
    #[serde(rename = "chain_rateideal5")]
    chain_rateideal_5: Option<i64>,
    #[serde(rename = "chain_rateideal6")]
    chain_rateideal_6: Option<f64>,
    #[serde(rename = "chain_rateideal7")]
    chain_rateideal_7: Option<f64>,
    #[serde(rename = "chain_rateideal8")]
    chain_rateideal_8: Option<f64>,
    #[serde(rename = "chain_rateideal9")]
    chain_rateideal_9: Option<i64>,
    #[serde(rename = "chain_rateideal10")]
    chain_rateideal_10: Option<i64>,
    #[serde(rename = "chain_rateideal11")]
    chain_rateideal_11: Option<i64>,
    #[serde(rename = "chain_rateideal12")]
    chain_rateideal_12: Option<i64>,
    #[serde(rename = "chain_rateideal13")]
    chain_rateideal_13: Option<i64>,
    #[serde(rename = "chain_rateideal14")]
    chain_rateideal_14: Option<i64>,
    #[serde(rename = "chain_rateideal15")]
    chain_rateideal_15: Option<i64>,
    #[serde(rename = "chain_rateideal16")]
    chain_rateideal_16: Option<i64>,
    temp_max: Option<i64>,
    #[serde(rename = "Device Hardware%")]
    device_hardware: f64,
    no_matching_work: Option<i64>,
    #[serde(rename = "chain_acn1")]
    chain_acn_1: Option<i64>,
    #[serde(rename = "chain_acn2")]
    chain_acn_2: Option<i64>,
    #[serde(rename = "chain_acn3")]
    chain_acn_3: Option<i64>,
    #[serde(rename = "chain_acn4")]
    chain_acn_4: Option<i64>,
    #[serde(rename = "chain_acn5")]
    chain_acn_5: Option<i64>,
    #[serde(rename = "chain_acn6")]
    chain_acn_6: Option<i64>,
    #[serde(rename = "chain_acn7")]
    chain_acn_7: Option<i64>,
    #[serde(rename = "chain_acn8")]
    chain_acn_8: Option<i64>,
    #[serde(rename = "chain_acn9")]
    chain_acn_9: Option<i64>,
    #[serde(rename = "chain_acn10")]
    chain_acn_10: Option<i64>,
    #[serde(rename = "chain_acn11")]
    chain_acn_11: Option<i64>,
    #[serde(rename = "chain_acn12")]
    chain_acn_12: Option<i64>,
    #[serde(rename = "chain_acn13")]
    chain_acn_13: Option<i64>,
    #[serde(rename = "chain_acn14")]
    chain_acn_14: Option<i64>,
    #[serde(rename = "chain_acn15")]
    chain_acn_15: Option<i64>,
    #[serde(rename = "chain_acn16")]
    chain_acn_16: Option<i64>,
    #[serde(rename = "chain_cores6")]
    chain_cores_6: Option<i64>,
    #[serde(rename = "chain_cores7")]
    chain_cores_7: Option<i64>,
    #[serde(rename = "chain_cores8")]
    chain_cores_8: Option<i64>,
    #[serde(rename = "chain_acs1")]
    chain_acs_1: Option<String>,
    #[serde(rename = "chain_acs2")]
    chain_acs_2: Option<String>,
    #[serde(rename = "chain_acs3")]
    chain_acs_3: Option<String>,
    #[serde(rename = "chain_acs4")]
    chain_acs_4: Option<String>,
    #[serde(rename = "chain_acs5")]
    chain_acs_5: Option<String>,
    #[serde(rename = "chain_acs6")]
    chain_acs_6: Option<String>,
    #[serde(rename = "chain_acs7")]
    chain_acs_7: Option<String>,
    #[serde(rename = "chain_acs8")]
    chain_acs_8: Option<String>,
    #[serde(rename = "chain_acs9")]
    chain_acs_9: Option<String>,
    #[serde(rename = "chain_acs10")]
    chain_acs_10: Option<String>,
    #[serde(rename = "chain_acs11")]
    chain_acs_11: Option<String>,
    #[serde(rename = "chain_acs12")]
    chain_acs_12: Option<String>,
    #[serde(rename = "chain_acs13")]
    chain_acs_13: Option<String>,
    #[serde(rename = "chain_acs14")]
    chain_acs_14: Option<String>,
    #[serde(rename = "chain_acs15")]
    chain_acs_15: Option<String>,
    #[serde(rename = "chain_acs16")]
    chain_acs_16: Option<String>,
    #[serde(rename = "chain_hw1")]
    chain_hw_1: Option<i64>,
    #[serde(rename = "chain_hw2")]
    chain_hw_2: Option<i64>,
    #[serde(rename = "chain_hw3")]
    chain_hw_3: Option<i64>,
    #[serde(rename = "chain_hw4")]
    chain_hw_4: Option<i64>,
    #[serde(rename = "chain_hw5")]
    chain_hw_5: Option<i64>,
    #[serde(rename = "chain_hw6")]
    chain_hw_6: Option<i64>,
    #[serde(rename = "chain_hw7")]
    chain_hw_7: Option<i64>,
    #[serde(rename = "chain_hw8")]
    chain_hw_8: Option<i64>,
    #[serde(rename = "chain_hw9")]
    chain_hw_9: Option<i64>,
    #[serde(rename = "chain_hw10")]
    chain_hw_10: Option<i64>,
    #[serde(rename = "chain_hw11")]
    chain_hw_11: Option<i64>,
    #[serde(rename = "chain_hw12")]
    chain_hw_12: Option<i64>,
    #[serde(rename = "chain_hw13")]
    chain_hw_13: Option<i64>,
    #[serde(rename = "chain_hw14")]
    chain_hw_14: Option<i64>,
    #[serde(rename = "chain_hw15")]
    chain_hw_15: Option<i64>,
    #[serde(rename = "chain_hw16")]
    chain_hw_16: Option<i64>,
    #[serde(rename = "chain_hwrate1")]
    chain_hwrate_1: Option<i64>,
    #[serde(rename = "chain_hwrate2")]
    chain_hwrate_2: Option<i64>,
    #[serde(rename = "chain_hwrate3")]
    chain_hwrate_3: Option<i64>,
    #[serde(rename = "chain_hwrate4")]
    chain_hwrate_4: Option<i64>,
    #[serde(rename = "chain_hwrate5")]
    chain_hwrate_5: Option<i64>,
    #[serde(rename = "chain_hwrate6")]
    chain_hwrate_6: Option<f64>,
    #[serde(rename = "chain_hwrate7")]
    chain_hwrate_7: Option<f64>,
    #[serde(rename = "chain_hwrate8")]
    chain_hwrate_8: Option<f64>,
    #[serde(rename = "chain_hwrate9")]
    chain_hwrate_9: Option<i64>,
    #[serde(rename = "chain_hwrate10")]
    chain_hwrate_10: Option<i64>,
    #[serde(rename = "chain_hwrate11")]
    chain_hwrate_11: Option<i64>,
    #[serde(rename = "chain_hwrate12")]
    chain_hwrate_12: Option<i64>,
    #[serde(rename = "chain_hwrate13")]
    chain_hwrate_13: Option<i64>,
    #[serde(rename = "chain_hwrate14")]
    chain_hwrate_14: Option<i64>,
    #[serde(rename = "chain_hwrate15")]
    chain_hwrate_15: Option<i64>,
    #[serde(rename = "chain_hwrate16")]
    chain_hwrate_16: Option<i64>,
    #[serde(rename = "chain_rate1")]
    chain_rate_1: Option<String>,
    #[serde(rename = "chain_rate2")]
    chain_rate_2: Option<String>,
    #[serde(rename = "chain_rate3")]
    chain_rate_3: Option<String>,
    #[serde(rename = "chain_rate4")]
    chain_rate_4: Option<String>,
    #[serde(rename = "chain_rate5")]
    chain_rate_5: Option<String>,
    #[serde(rename = "chain_rate6")]
    chain_rate_6: Option<String>,
    #[serde(rename = "chain_rate7")]
    chain_rate_7: Option<String>,
    #[serde(rename = "chain_rate8")]
    chain_rate_8: Option<String>,
    #[serde(rename = "chain_rate9")]
    chain_rate_9: Option<String>,
    #[serde(rename = "chain_rate10")]
    chain_rate_10: Option<String>,
    #[serde(rename = "chain_rate11")]
    chain_rate_11: Option<String>,
    #[serde(rename = "chain_rate12")]
    chain_rate_12: Option<String>,
    #[serde(rename = "chain_rate13")]
    chain_rate_13: Option<String>,
    #[serde(rename = "chain_rate14")]
    chain_rate_14: Option<String>,
    #[serde(rename = "chain_rate15")]
    chain_rate_15: Option<String>,
    #[serde(rename = "chain_rate16")]
    chain_rate_16: Option<String>,
    #[serde(rename = "chain_xtime6")]
    chain_xtime_6: Option<String>,
    #[serde(rename = "chain_xtime7")]
    chain_xtime_7: Option<String>,
    #[serde(rename = "chain_xtime8")]
    chain_xtime_8: Option<String>,
    chain_offside_6: Option<String>,
    chain_offside_7: Option<String>,
    chain_offside_8: Option<String>,
    chain_opencore_6: Option<String>,
    chain_opencore_7: Option<String>,
    chain_opencore_8: Option<String>,
    miner_version: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Status {
    #[serde(rename = "STATUS")]
    _status: String,
    #[serde(rename = "When")]
    _when: i64,
    #[serde(rename = "Code")]
    _code: i64,
    #[serde(rename = "Msg")]
    _msg: String,
    #[serde(rename = "Description")]
    _description: String,
}

#[derive(Serialize, Deserialize)]
struct Summary {
    #[serde(rename = "STATUS")]
    _status: Vec<Status>,
    #[serde(rename = "SUMMARY")]
    _summary: Vec<Summary1>,
    id: i64,
}

#[derive(Serialize, Deserialize)]
struct Summary1 {
    #[serde(rename = "Elapsed")]
    _elapsed: i64,
    #[serde(rename = "GHS 5s")]
    ghs_5_s: String,
    #[serde(rename = "GHS av")]
    ghs_av: f64,
    #[serde(rename = "Hashrate1m")]
    hashrate_1_m: f64,
    #[serde(rename = "Hashrate15m")]
    hashrate_15_m: f64,
    #[serde(rename = "Hashrate24h")]
    hashrate_24_h: f64,
    #[serde(rename = "Found Blocks")]
    found_blocks: i64,
    #[serde(rename = "Getworks")]
    _getworks: i64,
    #[serde(rename = "Accepted")]
    _accepted: i64,
    #[serde(rename = "Rejected")]
    _rejected: i64,
    #[serde(rename = "Hardware Errors")]
    hardware_errors: i64,
    #[serde(rename = "Utility")]
    _utility: f64,
    #[serde(rename = "Discarded")]
    _discarded: i64,
    #[serde(rename = "Stale")]
    _stale: i64,
    #[serde(rename = "Get Failures")]
    get_failures: i64,
    #[serde(rename = "Local Work")]
    local_work: i64,
    #[serde(rename = "Remote Failures")]
    remote_failures: i64,
    #[serde(rename = "Network Blocks")]
    network_blocks: i64,
    #[serde(rename = "Total MH")]
    total_mh: i64,
    #[serde(rename = "Work Utility")]
    work_utility: f64,
    #[serde(rename = "Difficulty Accepted")]
    difficulty_accepted: i64,
    #[serde(rename = "Difficulty Rejected")]
    difficulty_rejected: i64,
    #[serde(rename = "Difficulty Stale")]
    difficulty_stale: i64,
    #[serde(rename = "Best Share")]
    best_share: i64,
    #[serde(rename = "Device Hardware%")]
    device_hardware: f64,
    #[serde(rename = "Device Rejected%")]
    device_rejected: f64,
    #[serde(rename = "Pool Rejected%")]
    pool_rejected: f64,
    #[serde(rename = "Pool Stale%")]
    pool_stale: i64,
    #[serde(rename = "Last getwork")]
    last_getwork: i64,
}
