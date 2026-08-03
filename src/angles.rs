use crate::constants::{PI, TAU};

pub fn normalize_2pi(angle: f64) -> f64 {
    let rem: f64 = angle.rem_euclid(TAU);

    if rem == TAU {
        return 0.0;
    }

    rem
}

pub fn normalize_pi(angle: f64) -> f64 {
    let rem: f64 = normalize_2pi(angle);

    if rem >= PI {
        return rem - TAU;
    }

    rem
}
