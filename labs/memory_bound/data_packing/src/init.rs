use crate::{MAX_RANDOM, MIN_RANDOM, N, S};

pub fn create_entry(first_value: u8, second_value: u8) -> S {
    S {
        i: first_value,
        l: first_value as i16 * second_value as i16,
        s: second_value,
        d: first_value as f32 / MAX_RANDOM as f32,
        b: first_value < second_value,
    }
}

pub fn init() -> [S; N] {
    use rand::distributions::Uniform;
    use rand::prelude::*;
    let mut generator = thread_rng();
    let distribution = Uniform::from(MIN_RANDOM..MAX_RANDOM - 1);

    let mut out = [Default::default(); N];
    for i in 0..N {
        let random_int1 = distribution.sample(&mut generator);
        let random_int2 = distribution.sample(&mut generator);
        out[i] = create_entry(random_int1, random_int2);
    }
    out
}
