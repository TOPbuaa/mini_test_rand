use rand::Rng;
use rand::SeedableRng;

const SEED: u64 = 200;

pub struct RandGen {
    sum: u64,
    rng: rand::rngs::StdRng,
}

impl RandGen {
    pub fn new() -> Self {
        RandGen {
            sum: 0,
            rng: rand::rngs::StdRng::seed_from_u64(SEED),
        }
    }

    pub fn gen_rand_test(&mut self) {
        let mut sum = self.sum;
        for _ in 0..2 {
            let num = self.rng.gen_range(0..=50_000);
            println!("num: {}", num);
            sum += num;
        }
        self.sum = sum;
    }
}

fn main() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(SEED);
    for _ in 0..10 {
        println!("Integer: {}", rng.gen_range(0..=50_000));
    }
    let mut gen = RandGen::new();
    gen.gen_rand_test();
}
