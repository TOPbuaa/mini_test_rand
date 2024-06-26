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
        for i in 0..10 {
            let num = self.rng.gen_range(0..=50_000);
            println!("the {} num: {}", i, num);
            sum += num;
        }
        self.sum = sum;
    }
}

fn main() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(SEED);
    for i in 0..10 {
        println!("the {} num: {}", i, rng.gen_range(0..=50_000));
    }

    println!();
    let mut gen = RandGen::new();
    gen.gen_rand_test();
}
