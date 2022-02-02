use concrete::*;
use concrete_npe as npe;
use concrete_commons::numeric::Numeric;
//use concrete_core::math::tensor::Tensor;
use std::thread;
use std::sync::{Arc, mpsc};
use std::time::Instant;
//use rayon::prelude::*;

fn add_1(x: f64) -> f64{
    x+1.
}

fn lwe_pp(c: &Vec<LWE>) ->(){
    let padd = c[0].encoder.nb_bit_padding;
    let prec = c[0].encoder.nb_bit_precision;
    let noise = npe::nb_bit_from_variance_99(c[0].variance, <Torus as Numeric>::BITS as usize);  
    let nothing = <Torus as Numeric>::BITS - padd - prec - noise;

    println!("Paddign = {}, Precision = {}, Empty = {}, Noise = {}", padd, prec, nothing, noise);
}

fn sequential_bootstrap(f: fn(f64) -> f64, mut c_vec: Vec<LWE>, bsk: &LWEBSK, enc: &Encoder) -> Vec<LWE>{
    for lwe in c_vec.iter_mut(){
        *lwe = lwe.bootstrap_with_function(&bsk, |x| f(x), &enc).unwrap();
    }
    return c_vec
}
//fn slicing

// DOES NOT GUARANTEE ORDER OF CIPHERTEXT, 
fn concurrent_bootstrap(thread_count: usize, f: fn(f64) -> f64, c_vec: Vec<LWE>, bsk: &LWEBSK, enc: &Encoder) -> Vec<LWE>{

    let (tx, rx) = mpsc::channel();
    let mut threads = vec![];
    let mut vec = vec![];
   
    for i in 0..thread_count{
        
        let tx_clone = tx.clone();
        let enc_clone = enc.clone();
        let bsk_clone = bsk.clone();

        let size = c_vec.len()/thread_count; //as usize; 
        
        let mut work: Vec<LWE> = c_vec[i*size..(i+1)*size].to_vec();

        let t = thread::spawn( move || {
            for lwe_text in work.iter_mut(){
                *lwe_text = lwe_text.bootstrap_with_function(&bsk_clone, |x| f(x), &enc_clone).unwrap()            
                //*lwe_text = lwe_text.bootstrap_with_function(bsk, |x| f(x), enc).unwrap();
            }
            tx_clone.send(work).unwrap();
        
        });
        threads.push(t);
    }
    
    for t in threads{
        t.join().unwrap();
    }
    
    while let Ok(results) = rx.try_recv(){
        vec.push(results);
    }

    return vec.into_iter().flatten().collect::<Vec<LWE>>();
}

fn main() -> Result<(), CryptoAPIError> {

    let sk0 = LWESecretKey::new(&LWE80_650);
    let sk_rlwe = RLWESecretKey::new(&RLWE80_1024_1);
    let sk1 = sk_rlwe.to_lwe_secret_key();

    println!("Making Bootstrapping Key");
    let bsk = LWEBSK::new(&sk0, &sk_rlwe, 5, 4);    

    let enc = Encoder::new(0., 2., 4, 1).unwrap();

    let c = LWE::encode_encrypt(&sk0, 0., &enc).unwrap();

    let c_vec: Vec<LWE> = vec![c.clone(); 80];
    
    println!("Starting Parallel!");
    let time_start = Instant::now();
    let res_vec = concurrent_bootstrap(8, add_1, c_vec.clone(), &bsk, &enc);
    println!("{:.3}", (time_start.elapsed().as_micros() as f32)/ 1_000_000.);

    println!("Starting Sequential!");
    let time_start = Instant::now();
    let res_vec = sequential_bootstrap(add_1, c_vec.clone(), &bsk, &enc);
    println!("{:.3}", (time_start.elapsed().as_micros() as f32)/ 1_000_000.);

    lwe_pp(&c_vec);
    Ok(())
}
