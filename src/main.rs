extern crate rand;
extern crate rayon;

const RUNS: usize = 10000;
const BOUND: i32 = 100;

const STEPS: [i32; 2] = [-1, 1];

use rand::thread_rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;

fn do_step(n: &mut i32) {
    let mut rng = thread_rng();
    let choice = STEPS.choose(&mut rng);
    match choice {
        Some(x) => *n += x,
        None => println!("No choice made"),
    }
    // if rng.gen_bool(0.5) {
        // *n = *n + 1;
    // } else {
        // *n = *n - 1
    // }
}

fn average(numbers: std::vec::Vec<i32>) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn reach_bound(pos: &i32, time: &mut i32, cur_round: i32) {
    if *time == 0 && (*pos).abs() == BOUND {
        *time = cur_round;
    }
}

fn main() {
    let mut pos_v = vec![0; RUNS];
    let mut time_v = vec![0; RUNS];
    let mut min: i32 = 0;
    let mut round: i32 = 0;
    while min == 0 {
        // do steps
        pos_v.par_iter_mut().for_each(|n| do_step(n));
        round += 1;
        // update times
        pos_v.par_iter().zip(time_v.par_iter_mut()).for_each(|x| reach_bound(x.0, x.1, round));
        let result = time_v.par_iter().min();
        match result {
            Some(x) => min = *x,
            None => println!("Vec is somehow empty"),
        }
    }
    println!("{:?}", pos_v);
    println!("{:?}", time_v);
    println!("{}", average(time_v));
}
