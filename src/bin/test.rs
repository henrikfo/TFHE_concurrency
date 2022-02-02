mod functions;
use functions::*;

use std::time::Instant;
use concrete::*;


fn compare_parallel_boot() -> (){

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

    /*
    println!("Starting Sequential!");
    let time_start = Instant::now();
    let res_vec = sequential_bootstrap(add_1, c_vec.clone(), &bsk, &enc);
    println!("{:.3}", (time_start.elapsed().as_micros() as f32)/ 1_000_000.);
    */

    lwe_pp(&c_vec);
    return;
}

fn main() ->(){
    
    compare_parallel_boot();

    return
}
