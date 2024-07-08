use divan::{black_box, Bencher};
use faster_lcs::{algorithms::increasing_state_lcs::IncreasingStateLcs, lcs_trait::Lcs};
use rand::distributions::{Distribution, Uniform};

#[divan::bench]
fn subsequence(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let source: Vec<u8> = black_box(die.sample_iter(&mut rng).take(1000).collect());
    let target: Vec<u8> = black_box(die.sample_iter(&mut rng).take(1000).collect());
    bencher.bench_local(move || {
        black_box(IncreasingStateLcs::new(&source, &target).subsequence());
    });
}


#[divan::bench]
fn length(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let source: Vec<u8> = black_box(die.sample_iter(&mut rng).take(1000).collect());
    let target: Vec<u8> = black_box(die.sample_iter(&mut rng).take(1000).collect());
    bencher.bench_local(move || {
        black_box(IncreasingStateLcs::new(&source, &target).len());
    });
}