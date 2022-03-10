#![allow(unused_variables, dead_code)]
//mod lib;
//use lib::*;
use TFHE_concurrency::*;
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
    let mut tfhe = Tfheconcurrency::init_new(&LWE80_1024, &RLWE80_1024_1, Encoder::new(0., 2., 4, 1).unwrap(), 2, false);

    // Input: (Val: f64, Len: usize)
    // Get a Vector of Ciphertexts with value Val and length Len
    let c_vec = tfhe.get_ctxt_vec(0., 8);

    // Input: (Vector: Vec<LWE>, f: fn) 
    // Evaluate the function f on the Vector "Vector
    let _ = tfhe.para_boot(c_vec.clone(), plus_1);

    Ok(())
}
