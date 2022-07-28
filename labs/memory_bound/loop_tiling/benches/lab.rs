use criterion::{black_box, criterion_group, criterion_main, Criterion};

use loop_tiling::{init_matrix, original_solution, solution};

fn bench_loop_tiling(c: &mut Criterion) {
    let mut group = c.benchmark_group("loop_tiling");

    const N: usize = 2001;

    let mut matrix_in = vec![vec![0_f64; N]; N];
    init_matrix(&mut matrix_in);

    let mut matrix_out_original = vec![vec![0_f64; N]; N];
    let mut matrix_out_solution = vec![vec![0_f64; N]; N];

    group.bench_with_input("original_solution", &matrix_in, |b, input| {
        b.iter(|| {
            original_solution(input, &mut matrix_out_original);
            black_box(&mut matrix_out_original);
        })
    });

    group.bench_with_input("solution", &matrix_in, |b, input| {
        b.iter(|| {
            solution(input, &mut matrix_out_solution);
            black_box(&mut matrix_out_solution);
        })
    });
}

criterion_group!(benches, bench_loop_tiling);
criterion_main!(benches);
