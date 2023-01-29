// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut output: f64 = 221.0 * speed as f64;

    return match speed {
        1..=4 => output * 1.0,
        5..=8 => output * 0.9,
        9..=10 => output * 0.77,
        _ => output,
    };
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let mut output: u32 = production_rate_per_hour(speed) as u32;

    output / 60
}