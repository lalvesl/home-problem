use rand::{
    distr::{uniform::SampleUniform, Alphanumeric},
    rng, Rng,
};
use std::ops::Add;

#[allow(dead_code)]
pub fn gen_string(length: usize) -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn gen_num<T: Add<Output = T> + Copy + SampleUniform + PartialOrd>(
    min: T,
    max: T,
) -> T {
    rng().random_range(min..max)
}
