extern mod std;

use std::time::precise_time_s;

// This version makes ICE
//extern mod SciRust;
//use SciRust::matrix;

// We'll settle for this for now.
#[path="matrix/matrix.rs"]
mod matrix;

use matrix::{Matrix, TransposeMatrix, Create};
use matrix::generate::{identity, rand_L1};
use matrix::algorithms::{dot, mat_mul, transpose, cholesky_seq_inplace,
                        inverse, cholesky_blocked, par, mat_mul_blocked};
use matrix::util::to_str;

type M = Matrix<float>;

fn benchmark(N: uint) {
    io::println(fmt!("Benchmarking %? x %? matrices.", N, N));

    let L = rand_L1(N);
    let Lt = TransposeMatrix(&L);

    let start = precise_time_s();
    let A: M = mat_mul(&L, &Lt);
    let stop = precise_time_s();

    io::println(fmt!("Matrix Multiply: %?s", stop - start));

    let start = precise_time_s();
    let Ap: M = par::mat_mul(&L, &Lt);
    let stop = precise_time_s();

    io::println(fmt!("Matrix Multiply (parallel): %?s", stop - start));

    // TODO: make sure A and Ap agree.

    let start = precise_time_s();
    let Ab: M = mat_mul_blocked(&L, &Lt);
    let stop = precise_time_s();

    io::println(fmt!("Matrix Multiply (blocked): %?s", stop - start));

    let start = precise_time_s();
    let Ai: M = inverse(&A);
    let stop = precise_time_s();

    io::println(fmt!("Matrix Inverse: %?s", stop - start));

    let start = precise_time_s();
    let Ai: M = par::inverse(&A);
    let stop = precise_time_s();
    
    io::println(fmt!("Matrix Inverse (parallel): %?s", stop - start));

    let A2 = copy A;
    let start = precise_time_s();
    cholesky_seq_inplace::<float, M>(&A2);
    let stop = precise_time_s();

    io::println(fmt!("Cholesky (sequential): %?s", stop - start));    

    let start = precise_time_s();
    let Ac: M = cholesky_blocked(&A);
    let stop = precise_time_s();

    io::println(fmt!("Cholesky (blocked): %?s", stop - start));    

    let start = precise_time_s();
    let Ac: M = par::cholesky_blocked(&A);
    let stop = precise_time_s();

    io::println(fmt!("Cholesky (parallel): %?s", stop - start));    
}

fn main() {
    benchmark(1200);
}
