use rand::{rng, Rng};

pub fn random_variation(base: i32, range: i32) -> i32 {
    rng().random_range((base - range)..=(base + range))
}
