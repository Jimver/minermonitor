/*
Extract useful info from the probe result of a miner.
*/

use crate::backend;

pub struct AntS9 {
    hash_rate: f32,
    frequency: f32,
    voltage: f32,
    error_rate: f32,
    fan_speed: f32,
    temperature: f32
}

impl backend::api::miner::Miner for AntS9 {
    fn hash_rate(&self) -> f32 {
        self.hash_rate
    }

    fn frequency(&self) -> f32 {
        self.frequency
    }

    fn voltage(&self) -> f32 {
        self.voltage
    }

    fn error_rate(&self) -> f32 {
        self.error_rate
    }

    fn fan_speed(&self) -> f32 {
        self.fan_speed
    }

    fn temperature(&self) -> f32 {
        self.temperature
    }
}