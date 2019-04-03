/*
Extract useful info from the probe result of a miner.
*/

use crate::backend;
use crate::backend::probe::probe_result::{AntS9Probe, StatsLong, get_long_stats};

pub struct AntS9 {
    hash_rate: f64,
    frequency: Vec<i64>,
    voltage: Vec<f64>,
    error_rate: Vec<i64>,
    fan_speed: Vec<i64>,
    temperature1: Vec<i64>,
    temperature2: Vec<i64>
}

impl From<AntS9Probe> for AntS9 {
    fn from(probe: AntS9Probe) -> Self {
        let stats: StatsLong = get_long_stats(&probe);
        AntS9{
            hash_rate: probe.summary[0]._summary[0].ghs_5_s.parse().unwrap(),
            frequency: vec![stats.freq_avg_6.unwrap(), stats.freq_avg_7.unwrap(), stats.freq_avg_8.unwrap()],
            voltage: vec![stats.voltage_6.unwrap(), stats.voltage_7.unwrap(), stats.voltage_8.unwrap()],
            error_rate: vec![probe.devs[0]._devs[0].hardware_errors, probe.devs[0]._devs[1].hardware_errors, probe.devs[0]._devs[2].hardware_errors],
            fan_speed: vec![stats.fan_5.unwrap(), stats.fan_6.unwrap()],
            temperature1: vec![stats.temp_6.unwrap(), stats.temp_7.unwrap(), stats.temp_8.unwrap()],
            temperature2: vec![stats.temp_2_6.unwrap(), stats.temp_2_7.unwrap(), stats.temp_2_8.unwrap()]
        }
    }
}

impl backend::api::miner::Miner for AntS9 {
    fn hash_rate(&self) -> f64 {
        self.hash_rate
    }

    fn frequency(&self) -> Vec<i64> {
        self.frequency.clone()
    }

    fn voltage(&self) -> Vec<f64> {
        self.voltage.clone()
    }

    fn error_rate(&self) -> Vec<i64> {
        self.error_rate.clone()
    }

    fn fan_speed(&self) -> Vec<i64> {
        self.fan_speed.clone()
    }

    fn temperature1(&self) -> Vec<i64> {
        self.temperature1.clone()
    }

    fn temperature2(&self) -> Vec<i64> {
        self.temperature2.clone()
    }
}