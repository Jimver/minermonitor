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
    fn hash_rate(&self) -> f32;
    fn frequency(&self) -> f32;
    fn voltage(&self) -> f32;
    fn error_rate(&self) -> f32;
    fn fan_speed(&self) -> f32;
    fn temperature(&self) -> f32;
}