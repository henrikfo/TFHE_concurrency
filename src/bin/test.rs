#![allow(unused_variables, dead_code)]
mod lib;
use lib::*;
use concrete::*;
//use concrete_npe as npe;
//use concrete_commons::numeric::Numeric;
//use concrete_core::math::tensor::Tensor;
//use std::thread;
//use std::sync::mpsc;
//use std::marker::Send;
use std::time::Instant;

fn plus_1(x: f64) -> f64{
    x+1.
}

fn main() -> Result<(), CryptoAPIError> {

    // Input: (LWE Params: LWEParams, RLWE Params: RLWEParams, Max Threads: usize, Save: bool)
    // Initialize a TFHE instance
    
    let rlwe_params = RLWEParams {
        dimension: 1,
        polynomial_size: 4096,
        log2_std_dev: -62,
    };

    let mut tfhe = Tfheconcurrency::new(&LWE80_750, &rlwe_params, 2, false);
    //let mut tfhe = Tfheconcurrency::new(&LWE80_750, &RLWE80_2048_1, 2, false);

    // Input: (Val: f64, Len: usize)
    // Get a Vector of Ciphertexts with value Val and length Len
    //let lenght = vec![128, 256, 512, 1024];
    //let mut lenght = vec![];
    //for i in 2..32{
    //    lenght.push(i*128);
    //}
    //for len in lenght.iter(){
    let c_vec = tfhe.get_ctxt_vec(0., 4096);

    // Input: (Vector: Vec<LWE>, f: fn) 
    // Evaluate the function f on the Vector "Vector"
    let threads = vec![1, 8];
    let mut times = vec![];

    for thread in threads.iter(){ 
        tfhe.max_threads = *thread;
        let start = Instant::now();
        let _ = tfhe.para_boot(c_vec.clone(), plus_1);
        times.push((start.elapsed().as_millis() as f64) / 1000.);
    }
    println!("{}, {}, {}", 4096, times[0], times[1]);
    Ok(())
}
