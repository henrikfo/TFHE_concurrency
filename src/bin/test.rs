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
    let mut tfhe = Tfheconcurrency::new(&LWE80_256, &RLWE80_1024_1, 2, false);

    // Input: (Val: f64, Len: usize)
    // Get a Vector of Ciphertexts with value Val and length Len
    let lengths = vec![1*20,2*20,3*20,4*20,5*20,6*20,7*20,8*20,9*20,10*20,11*20,12*20,13*20,14*20,15*20,16*20];

    for nbr in lengths.iter(){

        let c_vec = tfhe.get_ctxt_vec(0., nbr.clone());

        // Input: (Vector: Vec<LWE>, f: fn) 
        // Evaluate the function f on the Vector "Vector"
        let mut times = vec![];
        let threads = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];

        for thread in threads.iter(){ 
            
            tfhe.max_threads = *thread;

            let start = Instant::now();
            
            for _ in 0..10{
                let _ = tfhe.para_boot(c_vec.clone(), plus_1);
            }
            times.push((start.elapsed().as_micros() as f32)/ 1_000_000.);
            //println!("{:.3}", (start.elapsed().as_micros() as f32)/ 1_000_000.);
            //println!("{}, {}", res_vec.len(), res_vec[0].decrypt_decode(&tfhe.sk1).unwrap());
        }

        let mut iter = times.iter().enumerate();
        let init = iter.next().ok_or("Need at least one input").unwrap();
        let res = iter.try_fold(init, |acc, x| {
            let cmp = x.1.partial_cmp(acc.1).unwrap();
            let min = if let std::cmp::Ordering::Less = cmp {
                x
            } else {
                acc
            };
            Some(min)
        });
        println!("nbr={}, fastest={:?}", nbr, res);
    }
    Ok(())
}
