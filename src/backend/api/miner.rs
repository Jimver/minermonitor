/*
File for miner api:
- GH/s (per asic)
- Frequency
- Voltage
- Error rate
- Fan speed (%)
- Temperature(s)
*/

pub trait Miner {
    fn hash_rate(&self) -> f64;
    fn frequency(&self) -> Vec<f64>;
    fn voltage(&self) -> Vec<f64>;
    fn error_rate(&self) -> Vec<i64>;
    fn fan_speed(&self) -> Vec<i64>;
    fn temperature1(&self) -> Vec<f64>;
    fn temperature2(&self) -> Vec<f64>;
}