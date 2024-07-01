use blstrs::{G1Projective, G2Projective};
use group::{Curve};
use std::time::{Duration, Instant};
use bench_msm::{prepare_input_g1, prepare_input_g2};
const RUNS: usize = 1000;

fn calculate_standard_deviation(durations: &[Duration], mean: Duration) -> Duration {
    let mean_nanos = mean.as_nanos() as f64;
    let variance = durations
        .iter()
        .map(|&duration| {
            let duration_nanos = duration.as_nanos() as f64;
            let diff = duration_nanos - mean_nanos;
            diff * diff
        })
        .sum::<f64>()
        / durations.len() as f64;
    let std_dev_nanos = variance.sqrt();
    Duration::from_nanos(std_dev_nanos as u64)
}

fn benchmark<F>(name: &str, sizes: &[u32], f: F)
where
    F: Fn(u32) -> (Duration, Duration),
{
    for &size in sizes {
        let mut msm_times = vec![];
        let mut naive_times = vec![];

        for _ in 0..RUNS {
            let (msm_time, naive_time) = f(size);
            msm_times.push(msm_time);
            naive_times.push(naive_time);
        }

        let avg_msm_time = msm_times.iter().sum::<Duration>() / (RUNS as u32);
        let avg_naive_time = naive_times.iter().sum::<Duration>() / (RUNS as u32);

        let msm_std_dev = calculate_standard_deviation(&msm_times, avg_msm_time);
        let naive_std_dev = calculate_standard_deviation(&naive_times, avg_naive_time);

        let naive_to_msm_ratio = (avg_naive_time.as_nanos() as f64) / (avg_msm_time.as_nanos() as f64);

        println!(
            "{} - Size: {} - MSM Average Time: {:?} - MSM Std Dev: {:?} - Naive Average Time: {:?} - Naive Std Dev: {:?} - Naive to MSM Ratio: {}",
            name, size, avg_msm_time, msm_std_dev, avg_naive_time, naive_std_dev, naive_to_msm_ratio
        );

        //println!(
        //    "| {} | {} | {:?} | {:?} | {:?} | {:?} | {:.2} |",
        //    name, size, avg_msm_time, msm_std_dev, avg_naive_time, naive_std_dev, naive_to_msm_ratio
        //);
    }
}

fn main() {
    let sizes = vec![10, 15, 20, 25, 30, 31, 32, 35, 40, 45, 50, 100, 200, 300, 400, 1000, 2000, 3000, 4000];

    benchmark("G1Projective", &sizes, |size| {
        let (g1_points, scalars) = prepare_input_g1(size);

        // Benchmark MSM approach
        let stopwatch = Instant::now();
        let result1: G1Projective = G1Projective::multi_exp(&g1_points, &scalars);
        let msm_time = stopwatch.elapsed();

        // Benchmark naive approach
        let stopwatch = Instant::now();
        let result2: G1Projective = g1_points
            .iter()
            .zip(scalars.iter())
            .map(|(g1, s)| g1 * s)
            .sum();
        let naive_time = stopwatch.elapsed();

        assert_eq!(result1.to_affine(), result2.to_affine());

        (msm_time, naive_time)
    });

    benchmark("G2Projective", &sizes, |size| {
        let (g2_points, scalars) = prepare_input_g2(size);

        // Benchmark MSM approach
        let stopwatch = Instant::now();
        let result1: G2Projective = G2Projective::multi_exp(&g2_points, &scalars);
        let msm_time = stopwatch.elapsed();

        // Benchmark naive approach
        let stopwatch = Instant::now();
        let result2: G2Projective = g2_points
            .iter()
            .zip(scalars.iter())
            .map(|(g2, s)| g2 * s)
            .sum();
        let naive_time = stopwatch.elapsed();

        assert_eq!(result1.to_affine(), result2.to_affine());

        (msm_time, naive_time)
    });
}
