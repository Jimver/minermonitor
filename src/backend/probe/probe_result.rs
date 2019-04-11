/*
Result of probing a miner
*/
use std::io;

use serde::{Deserialize, Serialize};
use serde_json::error::Error;

pub fn deserialize_string(string: String) -> AntS9Probe {
    serde_json::from_str(&string).unwrap()
}

pub fn deserialize_reader<R>(reader: R) -> Result<AntS9Probe, Error>
    where
        R: io::Read,
{
    serde_json::from_reader(reader)
}

pub fn get_short_stats(root: &AntS9Probe) -> StatsShort {
    StatsShort::from(root.stats[0]._stats[0].clone())
}

pub fn get_long_stats(root: &AntS9Probe) -> StatsLong {
    StatsLong::from(root.stats[0]._stats[1].clone())
}

// Root JSON object
#[derive(Clone, Serialize, Deserialize)]
pub struct AntS9Probe {
    pub stats: Vec<Stats>,
    pub pools: Vec<Pools>,
    pub summary: Vec<Summary>,
    pub devs: Vec<Devs>,
    pub fanctrl: Vec<Fanctrl>,
    pub id: i64,
}

// stats child
#[derive(Clone, Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "STATUS")]
    pub _status: Vec<Status>,
    #[serde(rename = "STATS")]
    pub _stats: Vec<StatsShortOrStatsLong>,
    pub id: i64,
}

// pools child
#[derive(Clone, Serialize, Deserialize)]
pub struct Pools {
    #[serde(rename = "STATUS")]
    pub _status: Vec<Status>,
    #[serde(rename = "POOLS")]
    pub _pools: Vec<PoolData>,
    pub id: i64,
}

// summary child
#[derive(Clone, Serialize, Deserialize)]
pub struct Summary {
    #[serde(rename = "STATUS")]
    pub _status: Vec<Status>,
    #[serde(rename = "SUMMARY")]
    pub _summary: Vec<SummaryData>,
    pub id: i64,
}

// devs child
#[derive(Clone, Serialize, Deserialize)]
pub struct Devs {
    #[serde(rename = "STATUS")]
    pub _status: Vec<Status>,
    #[serde(rename = "DEVS")]
    pub _devs: Vec<DevsData>,
    pub id: i64,
}

// fanctrl child
#[derive(Clone, Serialize, Deserialize)]
pub struct Fanctrl {
    #[serde(rename = "STATUS")]
    pub _status: Vec<Status>,
    #[serde(rename = "FANCTRL")]
    pub _fanctrl: Vec<FanctrlData>,
    pub id: i64,
}

// Shared STATUS object
#[derive(Clone, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "STATUS")]
    pub _status: String,
    #[serde(rename = "When")]
    pub _when: i64,
    #[serde(rename = "Code")]
    pub _code: i64,
    #[serde(rename = "Msg")]
    pub _msg: String,
    #[serde(rename = "Description")]
    pub _description: String,
}

// Different objects in the STATS array
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatsShortOrStatsLong {
    Long(StatsLong),
    Short(StatsShort),
}

// Convert from enum to short stats
impl From<StatsShortOrStatsLong> for StatsShort {
    fn from(stats: StatsShortOrStatsLong) -> Self {
        match stats {
            StatsShortOrStatsLong::Short(v) => v,
            _ => unreachable!(),
        }
    }
}

// Convert from enum to long stats
impl From<StatsShortOrStatsLong> for StatsLong {
    fn from(stats: StatsShortOrStatsLong) -> Self {
        match stats {
            StatsShortOrStatsLong::Long(v) => v,
            _ => unreachable!(),
        }
    }
}

// Pool data object
#[derive(Clone, Serialize, Deserialize)]
pub struct PoolData {
    #[serde(rename = "POOL")]
    pub _pool: i64,
    #[serde(rename = "URL")]
    pub _url: String,
    #[serde(rename = "Status")]
    pub _status: String,
    #[serde(rename = "Priority")]
    pub _priority: i64,
    #[serde(rename = "Quota")]
    pub _quota: i64,
    #[serde(rename = "Long Poll")]
    pub long_poll: String,
    #[serde(rename = "Getworks")]
    pub _getworks: i64,
    #[serde(rename = "Accepted")]
    pub _accepted: i64,
    #[serde(rename = "Rejected")]
    pub _rejected: i64,
    #[serde(rename = "Discarded")]
    pub _discarded: i64,
    #[serde(rename = "Stale")]
    pub _stale: i64,
    #[serde(rename = "Get Failures")]
    pub get_failures: i64,
    #[serde(rename = "Remote Failures")]
    pub remote_failures: i64,
    #[serde(rename = "User")]
    pub _user: String,
    #[serde(rename = "Last Share Time")]
    pub last_share_time: String,
    #[serde(rename = "Diff")]
    pub _diff: String,
    #[serde(rename = "LastDiff")]
    pub last_diff: f64,
    #[serde(rename = "Diff1 Shares")]
    pub diff_1_shares: i64,
    #[serde(rename = "Proxy Type")]
    proxy_type: String,
    #[serde(rename = "Proxy")]
    pub _proxy: String,
    #[serde(rename = "Difficulty Accepted")]
    pub difficulty_accepted: f64,
    #[serde(rename = "Difficulty Rejected")]
    pub difficulty_rejected: f64,
    #[serde(rename = "Difficulty Stale")]
    pub difficulty_stale: f64,
    #[serde(rename = "Last Share Difficulty")]
    pub last_share_difficulty: f64,
    #[serde(rename = "Has Stratum")]
    pub has_stratum: bool,
    #[serde(rename = "Asic Boost")]
    pub asic_boost: bool,
    #[serde(rename = "Stratum Active")]
    pub stratum_active: bool,
    #[serde(rename = "Stratum URL")]
    pub stratum_url: String,
    #[serde(rename = "Has GBT")]
    pub has_gbt: bool,
    #[serde(rename = "Best Share")]
    pub best_share: i64,
    #[serde(rename = "Pool Rejected%")]
    pub pool_rejected: f64,
    #[serde(rename = "Pool Stale%")]
    pub pool_stale: f64,
}

// Summary data object
#[derive(Clone, Serialize, Deserialize)]
pub struct SummaryData {
    #[serde(rename = "Elapsed")]
    pub _elapsed: i64,
    #[serde(rename = "GHS 5s")]
    pub ghs_5_s: String,
    #[serde(rename = "GHS av")]
    pub ghs_av: f64,
    #[serde(rename = "Hashrate1m")]
    pub hashrate_1_m: f64,
    #[serde(rename = "Hashrate15m")]
    pub hashrate_15_m: f64,
    #[serde(rename = "Hashrate24h")]
    pub hashrate_24_h: f64,
    #[serde(rename = "Found Blocks")]
    pub found_blocks: i64,
    #[serde(rename = "Getworks")]
    pub _getworks: i64,
    #[serde(rename = "Accepted")]
    pub _accepted: i64,
    #[serde(rename = "Rejected")]
    pub _rejected: i64,
    #[serde(rename = "Hardware Errors")]
    pub hardware_errors: i64,
    #[serde(rename = "Utility")]
    pub _utility: f64,
    #[serde(rename = "Discarded")]
    pub _discarded: i64,
    #[serde(rename = "Stale")]
    pub _stale: i64,
    #[serde(rename = "Get Failures")]
    pub get_failures: i64,
    #[serde(rename = "Local Work")]
    pub local_work: i64,
    #[serde(rename = "Remote Failures")]
    pub remote_failures: i64,
    #[serde(rename = "Network Blocks")]
    pub network_blocks: i64,
    #[serde(rename = "Total MH")]
    pub total_mh: f64,
    #[serde(rename = "Work Utility")]
    pub work_utility: f64,
    #[serde(rename = "Difficulty Accepted")]
    pub difficulty_accepted: f64,
    #[serde(rename = "Difficulty Rejected")]
    pub difficulty_rejected: f64,
    #[serde(rename = "Difficulty Stale")]
    pub difficulty_stale: f64,
    #[serde(rename = "Best Share")]
    pub best_share: i64,
    #[serde(rename = "Device Hardware%")]
    pub device_hardware: f64,
    #[serde(rename = "Device Rejected%")]
    pub device_rejected: f64,
    #[serde(rename = "Pool Rejected%")]
    pub pool_rejected: f64,
    #[serde(rename = "Pool Stale%")]
    pub pool_stale: f64,
    #[serde(rename = "Last getwork")]
    pub last_getwork: i64,
}

// Device (ASIC board) data object
#[derive(Clone, Serialize, Deserialize)]
pub struct DevsData {
    #[serde(rename = "ASC")]
    pub _asc: i64,
    #[serde(rename = "Name")]
    pub _name: String,
    #[serde(rename = "ID")]
    pub _id: i64,
    #[serde(rename = "Enabled")]
    pub _enabled: String,
    #[serde(rename = "Status")]
    pub _status: String,
    #[serde(rename = "TempAVG")]
    pub temp_avg: f64,
    #[serde(rename = "TempMAX")]
    pub temp_max: f64,
    #[serde(rename = "TempMIN")]
    pub temp_min: f64,
    #[serde(rename = "CHIP")]
    pub _chip: i64,
    #[serde(rename = "FREQ")]
    pub _freq: f64,
    #[serde(rename = "DUTY")]
    pub _duty: i64,
    #[serde(rename = "MHS av")]
    pub mhs_av: f64,
    #[serde(rename = "MHS 5s")]
    pub mhs_5_s: f64,
    #[serde(rename = "MHS 1m")]
    pub mhs_1_m: f64,
    #[serde(rename = "MHS 5m")]
    pub mhs_5_m: f64,
    #[serde(rename = "MHS 15m")]
    pub mhs_15_m: f64,
    #[serde(rename = "nominal MHS")]
    pub nominal_mhs: f64,
    #[serde(rename = "maximal MHS")]
    pub maximal_mhs: f64,
    #[serde(rename = "Accepted")]
    pub _accepted: i64,
    #[serde(rename = "Rejected")]
    pub _rejected: i64,
    #[serde(rename = "Hardware Errors")]
    pub hardware_errors: i64,
}

// Fan control data object
#[derive(Clone, Serialize, Deserialize)]
pub struct FanctrlData {
    #[serde(rename = "Mode")]
    pub _mode: String,
    #[serde(rename = "TargetTemp")]
    pub target_temp: f64,
    #[serde(rename = "TargetPwm")]
    pub target_pwm: i64,
    #[serde(rename = "Temperature")]
    pub _temperature: f64,
    #[serde(rename = "Output")]
    pub _output: i64,
    #[serde(rename = "Interval")]
    pub _interval: f64,
}

// The short status object
#[derive(Clone, Serialize, Deserialize)]
pub struct StatsShort {
    #[serde(rename = "BMMiner")]
    pub bm_miner: Option<String>,
    #[serde(rename = "Miner")]
    pub _miner: Option<String>,
    #[serde(rename = "CompileTime")]
    pub compile_time: Option<String>,
    #[serde(rename = "Type")]
    pub _type: Option<String>,
}

// The long status object
#[derive(Clone, Serialize, Deserialize)]
pub struct StatsLong {
    #[serde(rename = "STATS")]
    pub _stats: Option<i64>,
    #[serde(rename = "ID")]
    pub _id: Option<String>,
    #[serde(rename = "Elapsed")]
    pub _elapsed: Option<i64>,
    #[serde(rename = "Calls")]
    pub _calls: Option<i64>,
    #[serde(rename = "Wait")]
    pub _wait: Option<f64>,
    #[serde(rename = "Max")]
    pub _max: Option<f64>,
    #[serde(rename = "Min")]
    pub _min: Option<f64>,
    #[serde(rename = "GHS 5s")]
    pub ghs_5_s: String,
    #[serde(rename = "GHS av")]
    pub ghs_av: f64,
    pub miner_count: Option<i64>,
    pub frequency: Option<String>,
    pub fan_num: Option<i64>,
    #[serde(rename = "fan1")]
    pub fan_1: Option<i64>,
    #[serde(rename = "fan2")]
    pub fan_2: Option<i64>,
    #[serde(rename = "fan3")]
    pub fan_3: Option<i64>,
    #[serde(rename = "fan4")]
    pub fan_4: Option<i64>,
    #[serde(rename = "fan5")]
    pub fan_5: Option<i64>,
    #[serde(rename = "fan6")]
    pub fan_6: Option<i64>,
    #[serde(rename = "fan7")]
    pub fan_7: Option<i64>,
    #[serde(rename = "fan8")]
    pub fan_8: Option<i64>,
    #[serde(rename = "voltage1")]
    pub voltage_1: Option<f64>,
    #[serde(rename = "voltage2")]
    pub voltage_2: Option<f64>,
    #[serde(rename = "voltage3")]
    pub voltage_3: Option<f64>,
    #[serde(rename = "voltage4")]
    pub voltage_4: Option<f64>,
    #[serde(rename = "voltage5")]
    pub voltage_5: Option<f64>,
    #[serde(rename = "voltage6")]
    pub voltage_6: Option<f64>,
    #[serde(rename = "voltage7")]
    pub voltage_7: Option<f64>,
    #[serde(rename = "voltage8")]
    pub voltage_8: Option<f64>,
    #[serde(rename = "voltage9")]
    pub voltage_9: Option<f64>,
    #[serde(rename = "voltage10")]
    pub voltage_10: Option<f64>,
    #[serde(rename = "voltage11")]
    pub voltage_11: Option<f64>,
    #[serde(rename = "voltage12")]
    pub voltage_12: Option<f64>,
    #[serde(rename = "voltage13")]
    pub voltage_13: Option<f64>,
    #[serde(rename = "voltage14")]
    pub voltage_14: Option<f64>,
    #[serde(rename = "voltage15")]
    pub voltage_15: Option<f64>,
    #[serde(rename = "voltage16")]
    pub voltage_16: Option<f64>,
    pub temp_num: Option<i64>,
    #[serde(rename = "temp1")]
    pub temp_1: Option<f64>,
    #[serde(rename = "temp2")]
    pub temp_2: Option<f64>,
    #[serde(rename = "temp3")]
    pub temp_3: Option<f64>,
    #[serde(rename = "temp4")]
    pub temp_4: Option<f64>,
    #[serde(rename = "temp5")]
    pub temp_5: Option<f64>,
    #[serde(rename = "temp6")]
    pub temp_6: Option<f64>,
    #[serde(rename = "temp7")]
    pub temp_7: Option<f64>,
    #[serde(rename = "temp8")]
    pub temp_8: Option<f64>,
    #[serde(rename = "temp9")]
    pub temp_9: Option<f64>,
    #[serde(rename = "temp10")]
    pub temp_10: Option<f64>,
    #[serde(rename = "temp11")]
    pub temp_11: Option<f64>,
    #[serde(rename = "temp12")]
    pub temp_12: Option<f64>,
    #[serde(rename = "temp13")]
    pub temp_13: Option<f64>,
    #[serde(rename = "temp14")]
    pub temp_14: Option<f64>,
    #[serde(rename = "temp15")]
    pub temp_15: Option<f64>,
    #[serde(rename = "temp16")]
    pub temp_16: Option<f64>,
    #[serde(rename = "temp2_1")]
    pub temp_2_1: Option<f64>,
    #[serde(rename = "temp2_2")]
    pub temp_2_2: Option<f64>,
    #[serde(rename = "temp2_3")]
    pub temp_2_3: Option<f64>,
    #[serde(rename = "temp2_4")]
    pub temp_2_4: Option<f64>,
    #[serde(rename = "temp2_5")]
    pub temp_2_5: Option<f64>,
    #[serde(rename = "temp2_6")]
    pub temp_2_6: Option<f64>,
    #[serde(rename = "temp2_7")]
    pub temp_2_7: Option<f64>,
    #[serde(rename = "temp2_8")]
    pub temp_2_8: Option<f64>,
    #[serde(rename = "temp2_9")]
    pub temp_2_9: Option<f64>,
    #[serde(rename = "temp2_10")]
    pub temp_2_10: Option<f64>,
    #[serde(rename = "temp2_11")]
    pub temp_2_11: Option<f64>,
    #[serde(rename = "temp2_12")]
    pub temp_2_12: Option<f64>,
    #[serde(rename = "temp2_13")]
    pub temp_2_13: Option<f64>,
    #[serde(rename = "temp2_14")]
    pub temp_2_14: Option<f64>,
    #[serde(rename = "temp2_15")]
    pub temp_2_15: Option<f64>,
    #[serde(rename = "temp2_16")]
    pub temp_2_16: Option<f64>,
    #[serde(rename = "temp3_1")]
    pub temp_3_1: Option<f64>,
    #[serde(rename = "temp3_2")]
    pub temp_3_2: Option<f64>,
    #[serde(rename = "temp3_3")]
    pub temp_3_3: Option<f64>,
    #[serde(rename = "temp3_4")]
    pub temp_3_4: Option<f64>,
    #[serde(rename = "temp3_5")]
    pub temp_3_5: Option<f64>,
    #[serde(rename = "temp3_6")]
    pub temp_3_6: Option<f64>,
    #[serde(rename = "temp3_7")]
    pub temp_3_7: Option<f64>,
    #[serde(rename = "temp3_8")]
    pub temp_3_8: Option<f64>,
    #[serde(rename = "temp3_9")]
    pub temp_3_9: Option<f64>,
    #[serde(rename = "temp3_10")]
    pub temp_3_10: Option<f64>,
    #[serde(rename = "temp3_11")]
    pub temp_3_11: Option<f64>,
    #[serde(rename = "temp3_12")]
    pub temp_3_12: Option<f64>,
    #[serde(rename = "temp3_13")]
    pub temp_3_13: Option<f64>,
    #[serde(rename = "temp3_14")]
    pub temp_3_14: Option<f64>,
    #[serde(rename = "temp3_15")]
    pub temp_3_15: Option<f64>,
    #[serde(rename = "temp3_16")]
    pub temp_3_16: Option<f64>,
    #[serde(rename = "freq_desc6")]
    pub freq_desc_6: Option<String>,
    #[serde(rename = "freq_desc7")]
    pub freq_desc_7: Option<String>,
    #[serde(rename = "freq_desc8")]
    pub freq_desc_8: Option<String>,
    #[serde(rename = "freq_avg1")]
    pub freq_avg_1: Option<f64>,
    #[serde(rename = "freq_avg2")]
    pub freq_avg_2: Option<f64>,
    #[serde(rename = "freq_avg3")]
    pub freq_avg_3: Option<f64>,
    #[serde(rename = "freq_avg4")]
    pub freq_avg_4: Option<f64>,
    #[serde(rename = "freq_avg5")]
    pub freq_avg_5: Option<f64>,
    #[serde(rename = "freq_avg6")]
    pub freq_avg_6: Option<f64>,
    #[serde(rename = "freq_avg7")]
    pub freq_avg_7: Option<f64>,
    #[serde(rename = "freq_avg8")]
    pub freq_avg_8: Option<f64>,
    #[serde(rename = "freq_avg9")]
    pub freq_avg_9: Option<f64>,
    #[serde(rename = "freq_avg10")]
    pub freq_avg_10: Option<f64>,
    #[serde(rename = "freq_avg11")]
    pub freq_avg_11: Option<f64>,
    #[serde(rename = "freq_avg12")]
    pub freq_avg_12: Option<f64>,
    #[serde(rename = "freq_avg13")]
    pub freq_avg_13: Option<f64>,
    #[serde(rename = "freq_avg14")]
    pub freq_avg_14: Option<f64>,
    #[serde(rename = "freq_avg15")]
    pub freq_avg_15: Option<f64>,
    #[serde(rename = "freq_avg16")]
    pub req_avg_16: Option<f64>,
    pub total_rateideal: Option<f64>,
    pub total_freqavg: Option<f64>,
    pub total_acn: Option<i64>,
    pub total_rate: Option<f64>,
    #[serde(rename = "chain_rateideal1")]
    pub chain_rateideal_1: Option<f64>,
    #[serde(rename = "chain_rateideal2")]
    pub chain_rateideal_2: Option<f64>,
    #[serde(rename = "chain_rateideal3")]
    pub chain_rateideal_3: Option<f64>,
    #[serde(rename = "chain_rateideal4")]
    pub cjain_rateideal_4: Option<f64>,
    #[serde(rename = "chain_rateideal5")]
    pub cjain_rateideal_5: Option<f64>,
    #[serde(rename = "chain_rateideal6")]
    pub cjain_rateideal_6: Option<f64>,
    #[serde(rename = "chain_rateideal7")]
    pub cjain_rateideal_7: Option<f64>,
    #[serde(rename = "chain_rateideal8")]
    pub cjain_rateideal_8: Option<f64>,
    #[serde(rename = "chain_rateideal9")]
    pub cjain_rateideal_9: Option<f64>,
    #[serde(rename = "chain_rateideal10")]
    pub cjain_rateideal_10: Option<f64>,
    #[serde(rename = "chain_rateideal11")]
    pub cjain_rateideal_11: Option<f64>,
    #[serde(rename = "chain_rateideal12")]
    pub cjain_rateideal_12: Option<f64>,
    #[serde(rename = "chain_rateideal13")]
    pub cjain_rateideal_13: Option<f64>,
    #[serde(rename = "chain_rateideal14")]
    pub cjain_rateideal_14: Option<f64>,
    #[serde(rename = "chain_rateideal15")]
    pub cjain_rateideal_15: Option<f64>,
    #[serde(rename = "chain_rateideal16")]
    pub cjain_rateideal_16: Option<f64>,
    pub temp_max: Option<f64>,
    #[serde(rename = "Device Hardware%")]
    pub device_hardware: f64,
    pub no_matching_work: Option<i64>,
    #[serde(rename = "chain_acn1")]
    pub chain_acn_1: Option<i64>,
    #[serde(rename = "chain_acn2")]
    pub chain_acn_2: Option<i64>,
    #[serde(rename = "chain_acn3")]
    pub chain_acn_3: Option<i64>,
    #[serde(rename = "chain_acn4")]
    pub chain_acn_4: Option<i64>,
    #[serde(rename = "chain_acn5")]
    pub chain_acn_5: Option<i64>,
    #[serde(rename = "chain_acn6")]
    pub chain_acn_6: Option<i64>,
    #[serde(rename = "chain_acn7")]
    pub chain_acn_7: Option<i64>,
    #[serde(rename = "chain_acn8")]
    pub chain_acn_8: Option<i64>,
    #[serde(rename = "chain_acn9")]
    pub chain_acn_9: Option<i64>,
    #[serde(rename = "chain_acn10")]
    pub chain_acn_10: Option<i64>,
    #[serde(rename = "chain_acn11")]
    pub chain_acn_11: Option<i64>,
    #[serde(rename = "chain_acn12")]
    pub chain_acn_12: Option<i64>,
    #[serde(rename = "chain_acn13")]
    pub chain_acn_13: Option<i64>,
    #[serde(rename = "chain_acn14")]
    pub chain_acn_14: Option<i64>,
    #[serde(rename = "chain_acn15")]
    pub chain_acn_15: Option<i64>,
    #[serde(rename = "chain_acn16")]
    pub chain_acn_16: Option<i64>,
    #[serde(rename = "chain_cores6")]
    pub chain_cores_6: Option<i64>,
    #[serde(rename = "chain_cores7")]
    pub chain_cores_7: Option<i64>,
    #[serde(rename = "chain_cores8")]
    pub chain_cores_8: Option<i64>,
    #[serde(rename = "chain_acs1")]
    pub chain_acs_1: Option<String>,
    #[serde(rename = "chain_acs2")]
    pub chain_acs_2: Option<String>,
    #[serde(rename = "chain_acs3")]
    pub chain_acs_3: Option<String>,
    #[serde(rename = "chain_acs4")]
    pub chain_acs_4: Option<String>,
    #[serde(rename = "chain_acs5")]
    pub chain_acs_5: Option<String>,
    #[serde(rename = "chain_acs6")]
    pub chain_acs_6: Option<String>,
    #[serde(rename = "chain_acs7")]
    pub chain_acs_7: Option<String>,
    #[serde(rename = "chain_acs8")]
    pub chain_acs_8: Option<String>,
    #[serde(rename = "chain_acs9")]
    pub chain_acs_9: Option<String>,
    #[serde(rename = "chain_acs10")]
    pub chain_acs_10: Option<String>,
    #[serde(rename = "chain_acs11")]
    pub chain_acs_11: Option<String>,
    #[serde(rename = "chain_acs12")]
    pub chain_acs_12: Option<String>,
    #[serde(rename = "chain_acs13")]
    pub chain_acs_13: Option<String>,
    #[serde(rename = "chain_acs14")]
    pub chain_acs_14: Option<String>,
    #[serde(rename = "chain_acs15")]
    pub chain_acs_15: Option<String>,
    #[serde(rename = "chain_acs16")]
    pub chain_acs_16: Option<String>,
    #[serde(rename = "chain_hw1")]
    pub chain_hw_1: Option<i64>,
    #[serde(rename = "chain_hw2")]
    pub chain_hw_2: Option<i64>,
    #[serde(rename = "chain_hw3")]
    pub chain_hw_3: Option<i64>,
    #[serde(rename = "chain_hw4")]
    pub chain_hw_4: Option<i64>,
    #[serde(rename = "chain_hw5")]
    pub chain_hw_5: Option<i64>,
    #[serde(rename = "chain_hw6")]
    pub chain_hw_6: Option<i64>,
    #[serde(rename = "chain_hw7")]
    pub chain_hw_7: Option<i64>,
    #[serde(rename = "chain_hw8")]
    pub chain_hw_8: Option<i64>,
    #[serde(rename = "chain_hw9")]
    pub chain_hw_9: Option<i64>,
    #[serde(rename = "chain_hw10")]
    pub chain_hw_10: Option<i64>,
    #[serde(rename = "chain_hw11")]
    pub chain_hw_11: Option<i64>,
    #[serde(rename = "chain_hw12")]
    pub chain_hw_12: Option<i64>,
    #[serde(rename = "chain_hw13")]
    pub chain_hw_13: Option<i64>,
    #[serde(rename = "chain_hw14")]
    pub chain_hw_14: Option<i64>,
    #[serde(rename = "chain_hw15")]
    pub chain_hw_15: Option<i64>,
    #[serde(rename = "chain_hw16")]
    pub chain_hw_16: Option<i64>,
    #[serde(rename = "chain_hwrate1")]
    pub chain_hwrate_1: Option<f64>,
    #[serde(rename = "chain_hwrate2")]
    pub chain_hwrate_2: Option<f64>,
    #[serde(rename = "chain_hwrate3")]
    pub chain_hwrate_3: Option<f64>,
    #[serde(rename = "chain_hwrate4")]
    pub chain_hwrate_4: Option<f64>,
    #[serde(rename = "chain_hwrate5")]
    pub chain_hwrate_5: Option<f64>,
    #[serde(rename = "chain_hwrate6")]
    pub chain_hwrate_6: Option<f64>,
    #[serde(rename = "chain_hwrate7")]
    pub chain_hwrate_7: Option<f64>,
    #[serde(rename = "chain_hwrate8")]
    pub chain_hwrate_8: Option<f64>,
    #[serde(rename = "chain_hwrate9")]
    pub chain_hwrate_9: Option<f64>,
    #[serde(rename = "chain_hwrate10")]
    pub chain_hwrate_10: Option<f64>,
    #[serde(rename = "chain_hwrate11")]
    pub chain_hwrate_11: Option<f64>,
    #[serde(rename = "chain_hwrate12")]
    pub chain_hwrate_12: Option<f64>,
    #[serde(rename = "chain_hwrate13")]
    pub chain_hwrate_13: Option<f64>,
    #[serde(rename = "chain_hwrate14")]
    pub chain_hwrate_14: Option<f64>,
    #[serde(rename = "chain_hwrate15")]
    pub chain_hwrate_15: Option<f64>,
    #[serde(rename = "chain_hwrate16")]
    pub chain_hwrate_16: Option<f64>,
    #[serde(rename = "chain_rate1")]
    pub chain_rate_1: Option<String>,
    #[serde(rename = "chain_rate2")]
    pub chain_rate_2: Option<String>,
    #[serde(rename = "chain_rate3")]
    pub chain_rate_3: Option<String>,
    #[serde(rename = "chain_rate4")]
    pub chain_rate_4: Option<String>,
    #[serde(rename = "chain_rate5")]
    pub chain_rate_5: Option<String>,
    #[serde(rename = "chain_rate6")]
    pub chain_rate_6: Option<String>,
    #[serde(rename = "chain_rate7")]
    pub chain_rate_7: Option<String>,
    #[serde(rename = "chain_rate8")]
    pub chain_rate_8: Option<String>,
    #[serde(rename = "chain_rate9")]
    pub chain_rate_9: Option<String>,
    #[serde(rename = "chain_rate10")]
    pub chain_rate_10: Option<String>,
    #[serde(rename = "chain_rate11")]
    pub chain_rate_11: Option<String>,
    #[serde(rename = "chain_rate12")]
    pub chain_rate_12: Option<String>,
    #[serde(rename = "chain_rate13")]
    pub chain_rate_13: Option<String>,
    #[serde(rename = "chain_rate14")]
    pub chain_rate_14: Option<String>,
    #[serde(rename = "chain_rate15")]
    pub chain_rate_15: Option<String>,
    #[serde(rename = "chain_rate16")]
    pub chain_rate_16: Option<String>,
    #[serde(rename = "chain_xtime6")]
    pub chain_xtime_6: Option<String>,
    #[serde(rename = "chain_xtime7")]
    pub chain_xtime_7: Option<String>,
    #[serde(rename = "chain_xtime8")]
    pub chain_xtime_8: Option<String>,
    pub chain_offside_6: Option<String>,
    pub chain_offside_7: Option<String>,
    pub chain_offside_8: Option<String>,
    pub chain_opencore_6: Option<String>,
    pub chain_opencore_7: Option<String>,
    pub chain_opencore_8: Option<String>,
    pub miner_version: Option<String>,
}
