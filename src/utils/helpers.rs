use rand::Rng;

pub fn generate_random_6_digits() -> u32 {
    rand::thread_rng().gen_range(100_000..1_000_000)
}
