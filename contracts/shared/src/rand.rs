use soroban_sdk::{Env, Bytes};

pub struct RandomNumberGenerator;

pub trait RandomNumberGeneratorTrait {
    fn new(env: &Env, seed: u64) -> Self;
    fn number(&mut self, env: &Env, max_range: u32) -> u32;
}

impl RandomNumberGeneratorTrait for RandomNumberGenerator {
    fn new(env: &Env, seed: u64) -> Self {
        let mut arr = [0u8; 32];
        let seed_bytes = seed.to_be_bytes();

        arr[24..32].copy_from_slice(&seed_bytes[0..8]);
        env.prng().seed(Bytes::from_slice(env, arr.as_slice()));
        RandomNumberGenerator{}
    }

    fn number(&mut self, env: &Env, max_range: u32) -> u32 {
        env.prng().gen_range::<u64>(1..=max_range as u64) as u32
    }
}
