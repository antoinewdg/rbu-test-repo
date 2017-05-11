#[macro_use]
extern crate bencher;
extern crate rbu_test_repo;

use bencher::Bencher;
use rbu_test_repo::sum_arrays;

fn bench_array_sum(bench: &mut Bencher) {
    const N: usize = 300000;
    let a = vec![1; N];
    let b = (0..N as i32).collect::<Vec<_>>();
    let mut c = vec![0; N];

    bench.iter(|| {
        sum_arrays(&a, &b, &mut c);
    })
}


benchmark_group!(benches, bench_array_sum);
benchmark_main!(benches);