use mem_sweep::{Work, can_process};
use std::io::{BufRead, stdin};

fn main() {
    let mut lines = stdin().lock().lines().map(Result::unwrap);

    let first = lines
        .next()
        .expect("Expected first line with `N memory_limit`");
    let mut parts = first.split_whitespace();
    let n: usize = parts
        .next()
        .expect("Missing N")
        .parse()
        .expect("Failed to parse N as usize");
    let memory_limit: u64 = parts
        .next()
        .expect("Missing memory_limit")
        .parse()
        .expect("Failed to parse memory_limit as i64");

    let mut work = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines
            .next()
            .unwrap_or_else(|| panic!("Expected {} lines of work data", n));
        let mut p = line.split_whitespace();
        let mem_usage: i64 = p
            .next()
            .expect("Missing mem_usage")
            .parse()
            .expect("Failed to parse mem_usage as i64");
        let start: u64 = p
            .next()
            .expect("Missing start")
            .parse()
            .expect("Failed to parse start as u64");
        let duration: u64 = p
            .next()
            .expect("Missing duration")
            .parse()
            .expect("Failed to parse duration as u64");

        work.push(Work {
            mem_usage,
            start,
            duration,
        });
    }

    if can_process(memory_limit, work) {
        println!("YES, can process");
    } else {
        println!("NO, can't process");
    }
}
