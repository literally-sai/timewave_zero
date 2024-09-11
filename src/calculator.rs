use crate::data::{HUANG_TI_DATA, KELLEY_DATA, NUM_DATA_POINTS, SHELIAK_DATA, WATKINS_DATA};
use rayon::prelude::*;

const NUM_POWERS: usize = 64;

pub struct TimewaveResult {
    pub dtz: f64,
    pub kelley: f64,
    pub watkins: f64,
    pub sheliak: f64,
    pub huang_ti: f64,
}

pub fn calculate_timewave(
    dtz: f64,
    step: f64,
    negative_bailout: f64,
    wave_factor: u32,
) -> Vec<TimewaveResult> {
    let powers = calculate_powers(wave_factor);
    let dtz_range = generate_dtz_range(dtz, step, negative_bailout);

    if cfg!(feature = "threaded") {
        dtz_range
            .into_par_iter()
            .map(|dtz| calculate_point(dtz, &powers))
            .collect()
    } else {
        dtz_range
            .into_iter()
            .map(|dtz| calculate_point(dtz, &powers))
            .collect()
    }
}

fn calculate_point(dtz: f64, powers: &[f64; NUM_POWERS]) -> TimewaveResult {
    TimewaveResult {
        dtz,
        kelley: f(dtz, &KELLEY_DATA, powers),
        watkins: f(dtz, &WATKINS_DATA, powers),
        sheliak: f(dtz, &SHELIAK_DATA, powers),
        huang_ti: f(dtz, &HUANG_TI_DATA, powers),
    }
}

fn f(x: f64, data: &[i32; NUM_DATA_POINTS], powers: &[f64; NUM_POWERS]) -> f64 {
    let mut sum = 0.0;

    if x != 0.0 {
        for i in 0..NUM_POWERS {
            if x >= powers[i] {
                sum += mult_power(v(div_power(x, i, powers), data), i, powers);
            }
        }

        let mut i = 0;
        loop {
            i += 1;
            if i > 1_000_002 {
                break;
            }
            let last_sum = sum;
            sum += div_power(v(mult_power(x, i, powers), data), i, powers);
            if sum == 0.0 || sum <= last_sum {
                break;
            }
        }
    }

    div_power(sum, 3, powers)
}

fn v(y: f64, data: &[i32; NUM_DATA_POINTS]) -> f64 {
    let i = (y % NUM_DATA_POINTS as f64) as usize;
    let j = (i + 1) % NUM_DATA_POINTS;
    let z = y - y.floor();

    if z == 0.0 {
        data[i] as f64
    } else {
        (data[j] - data[i]) as f64 * z + data[i] as f64
    }
}

fn calculate_powers(wave_factor: u32) -> [f64; NUM_POWERS] {
    let mut powers = [0.0; NUM_POWERS];
    powers[0] = 1.0;
    for i in 1..NUM_POWERS {
        powers[i] = powers[i - 1] * wave_factor as f64;
    }
    powers
}

fn mult_power(x: f64, i: usize, powers: &[f64; NUM_POWERS]) -> f64 {
    x * powers[i]
}

fn div_power(x: f64, i: usize, powers: &[f64; NUM_POWERS]) -> f64 {
    x / powers[i]
}

fn generate_dtz_range(dtz: f64, step: f64, negative_bailout: f64) -> Vec<f64> {
    let mut current = dtz;
    let mut range = Vec::new();
    while current >= negative_bailout {
        range.push(current);
        current -= step;
    }
    range
}
