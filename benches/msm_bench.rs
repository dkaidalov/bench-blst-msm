use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use bench_msm::{prepare_input_g1, prepare_input_g2};
use blstrs::{G1Projective, G2Projective, Scalar};

fn bench_msm(c: &mut Criterion) {

    let sizes: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41];
    //let sizes: Vec<usize> = vec![1, 2, 16, 31, 32, 41];

    let mut group1 = c.benchmark_group("msm_g1");
    let (g1_points, g1_scalars) = prepare_input_g1((*sizes.iter().max().unwrap()) as u32);

    for size in sizes.iter() {
        let (points, scalars) = (&g1_points[..*size], &g1_scalars[..*size]);
        let result_g1: G1Projective = G1Projective::multi_exp(points, scalars);

        group1.bench_with_input(
            BenchmarkId::new("blst_msm_g1", size),
            &(points, scalars, result_g1),
            |b, input| {
                b.iter(|| {
                    assert_eq!(input.2, G1Projective::multi_exp(input.0, input.1))
                });
            },
        );

        group1.bench_with_input(
            BenchmarkId::new("naive_msm_g1", size),
            &(points, scalars, result_g1),
            |b, input| {
                b.iter(|| {
                    let res: G1Projective = input.0
                        .iter()
                        .zip(input.1.iter())
                        .map(|(g1, s)| g1 * s)
                        .sum();
                    assert_eq!(input.2, res)
                });
            },
        );
    }
    group1.finish();

    let mut group2 = c.benchmark_group("msm_g2");
    let (g2_points, g2_scalars) = prepare_input_g2((*sizes.iter().max().unwrap()) as u32);

    for size in sizes.iter() {
        let (points, scalars) = (&g2_points[..*size], &g2_scalars[..*size]);
        let result_g2: G2Projective = G2Projective::multi_exp(points, scalars);

        group2.bench_with_input(
            BenchmarkId::new("blst_msm_g2", size),
            &(points, scalars, result_g2),
            |b, input| {
                b.iter(|| {
                    assert_eq!(input.2, G2Projective::multi_exp(input.0, input.1))
                });
            },
        );

        group2.bench_with_input(
            BenchmarkId::new("naive_msm_g2", size),
            &(points, scalars, result_g2),
            |b, input| {
                b.iter(|| {
                    let res: G2Projective = input.0
                        .iter()
                        .zip(input.1.iter())
                        .map(|(g1, s)| g1 * s)
                        .sum();
                    assert_eq!(input.2, res)
                });
            },
        );
    }
    group2.finish();
}

criterion_group!(
    benches,
    bench_msm,
);
criterion_main!(benches);