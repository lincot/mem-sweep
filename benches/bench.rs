#![feature(test)]

extern crate test;
use mem_sweep::{can_process, Job};
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench_30(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let n = 30;
    let jobs: Vec<_> = (0..n)
        .map(|_| Job {
            mem_usage: 1,
            start: rng.random_range(0..n),
            duration: rng.random_range(0..n / 10),
        })
        .collect();

    bencher.iter(|| can_process(black_box(10_000_000), black_box(jobs.iter().copied())));
}

#[bench]
fn bench_1000(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let n = 1000;
    let jobs: Vec<_> = (0..n)
        .map(|_| Job {
            mem_usage: 1,
            start: rng.random_range(0..n),
            duration: rng.random_range(0..n / 10),
        })
        .collect();

    bencher.iter(|| can_process(black_box(10_000_000), black_box(jobs.iter().copied())));
}

#[bench]
fn bench_2000(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let n = 2000;
    let jobs: Vec<_> = (0..n)
        .map(|_| Job {
            mem_usage: 1,
            start: rng.random_range(0..n),
            duration: rng.random_range(0..n / 10),
        })
        .collect();

    bencher.iter(|| can_process(black_box(10_000_000), black_box(jobs.iter().copied())));
}

#[bench]
fn bench_1000000(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let n = 1_000_000;
    let jobs: Vec<_> = (0..n)
        .map(|_| Job {
            mem_usage: 1,
            start: rng.random_range(0..n),
            duration: rng.random_range(0..n / 10),
        })
        .collect();

    bencher.iter(|| can_process(black_box(10_000_000), black_box(jobs.iter().copied())));
}
