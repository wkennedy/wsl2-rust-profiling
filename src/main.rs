use std::time::{Instant};

fn main() {
    nested_loop_one();
    single_loop();
    nested_loop_two();
}

fn single_loop() {
    for _n in 0..100 {
        let instant = Instant::now();
        let result = &format!("{}{:?}", "Time now: ", instant);
        println!("{}", result);
    }
}

fn nested_loop_one() {
    for _n in 0..2000 {
        for _i in 0..2000 {
            let instant = Instant::now();
            let result = &format!("{}{:?}", "Time now: ", instant);
            println!("{}", result);
        }
    }
}

fn nested_loop_two() {
    for _n in 0..2000 {
        for _i in 0..2000 {
            let instant = Instant::now();
            let result = &format!("{}{:?}", "Time now: ", instant);
            println!("{}", result);
        }
    }
}