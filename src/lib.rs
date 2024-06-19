use blstrs::{G1Projective, G2Projective, Scalar};
use rayon::iter::IntoParallelIterator;
use rand::rngs::StdRng;
use rand::{thread_rng, SeedableRng};
use rayon::prelude::*;
use ff::Field;
use group::Group;

pub fn prepare_input_g1(size: u32) -> (Vec<G1Projective>, Vec<Scalar>) {
    let mut rng = thread_rng();

    // Generate random scalars
    let scalars: Vec<Scalar> = (0..size).map(|_| Scalar::random(&mut rng)).collect();

    // Generate random G1 points
    let g1_points: Vec<G1Projective> = (0..size)
        .into_par_iter()
        .map(|i| {
            let mut thread_rng = StdRng::seed_from_u64(i as u64);
            G1Projective::random(&mut thread_rng)
        })
        .collect();
    (g1_points, scalars)
}

pub fn prepare_input_g2(size: u32) -> (Vec<G2Projective>, Vec<Scalar>) {
    let mut rng = thread_rng();

    // Generate random scalars
    let scalars: Vec<Scalar> = (0..size).map(|_| Scalar::random(&mut rng)).collect();

    // Generate random G2 points
    let g2_points: Vec<G2Projective> = (0..size)
        .into_par_iter()
        .map(|i| {
            let mut thread_rng = StdRng::seed_from_u64(i as u64);
            G2Projective::random(&mut thread_rng)
        })
        .collect();
    (g2_points, scalars)
}