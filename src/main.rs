extern crate rand;

// const BOUND: i32 = 1e3;
// const RUNS: i32 = 1e6;
const BOUND: i32 = 10000;

use rand::{thread_rng, Rng};

fn do_step(n: i32) -> i32 {
    let mut rng = thread_rng();
    if rng.gen_bool(0.5) {
        return n + 1;
    }
    n - 1
}

fn main() {
    let mut pos: i32 = 0;
    let mut rounds = 0;
    while pos.abs() != BOUND {
        // println!("{}", pos);
        pos = do_step(pos);
        rounds = rounds + 1;
    }
    println!("{}", rounds);
}
