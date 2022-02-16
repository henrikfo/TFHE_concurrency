use concrete::*;
use concrete_npe as npe;
use concrete_commons::numeric::Numeric;
//use concrete_core::math::tensor::Tensor;
use std::thread;
use std::sync::mpsc;


pub fn lwe_pp(lwe_vec: &Vec<LWE>) ->(){
    for lwe in lwe_vec.iter(){
        let padd = lwe.encoder.nb_bit_padding;
        let prec = lwe.encoder.nb_bit_precision;
        let noise = npe::nb_bit_from_variance_99(lwe.variance, <Torus as Numeric>::BITS as usize);  
        let nothing = <Torus as Numeric>::BITS - padd - prec - noise;
        println!("Paddign = {}, Precision = {}, Empty = {}, Noise = {}", padd, prec, nothing, noise);
    }
}
//fn slicing

pub struct Tfheconcurrency{
    pub sk0: LWESecretKey,
    pub sk1: LWESecretKey,
    bsk: LWEBSK,
    ksk:LWEKSK,
    enc: Encoder,
    //f: Fn(f64)->f64,
    pub max_threads: usize
}

impl Tfheconcurrency {
    pub fn init_new(lwe_par: &LWEParams, rlwe_par: &RLWEParams, enc: Encoder, threads: usize, _save: bool) -> Tfheconcurrency{

        println!("Making Keys!");
        let sk = LWESecretKey::new(lwe_par);
        let sk_rlwe = RLWESecretKey::new(rlwe_par);
        let sk_out = sk_rlwe.to_lwe_secret_key();

        /*if save{
            self.sk0.save();
            self.sk1.save();
            self.bsk.save();
            self.enc.save();
        }*/
        return Tfheconcurrency{
            bsk: LWEBSK::new(&sk, &sk_rlwe, 6, 6),
            ksk: LWEKSK::new(&sk_out, &sk, 6, 6)
            sk0: sk,
            sk1: sk_out
            enc: enc,
            max_threads: threads
            };
    }

    pub fn new(sk: LWESecretKey, sk_out: LWE, bsk: LWEBSK, ksk: LWEKSK, enc: Encoder, threads: usize, _save: bool) -> Tfheconcurrency{
        return Tfheconcurrency{
            bsk: bsk,
            ksk: ksk
            sk0: sk,
            sk1: sk_out
            enc: enc,
            max_threads: threads
            };
    }
    
    fn get_ctxt(&self, x: f64)->LWE{
        return LWE::encode_encrypt(&self.sk0, x, &self.enc).unwrap();
    }

    pub fn get_ctxt_vec(&self, x: f64, len: usize) -> Vec<LWE>{
        return vec![self.get_ctxt(x); len];
    }

    pub fn get_plain(&self, lwe: Vec<LWE>) -> f64{
        // Using the OUT key, need a keyswitch to be able to use the original key
        return lwe[0].decrypt_decode(&self.sk1).unwrap();
    }

    pub fn set_enc(&mut self, min: f64, max: f64, precision: usize, padding: usize) -> (){
        self.enc = Encoder::new(min, max, precision, padding).unwrap();   
    }
    
    pub fn seq_boot(&self, mut lwe_vec: Vec<LWE>, func: fn(f64) -> f64) -> Vec<LWE>{
        for lwe in lwe_vec.iter_mut(){
            *lwe = lwe.bootstrap_with_function(&self.bsk, |x| func(x), &self.enc).unwrap();
        }
        return lwe_vec
    }

    pub fn slicing(&self, length: usize, threads: &usize) -> Vec<usize>{
        let mut index = vec![0; threads.clone()];
        let mut indexes = vec![0];
        for i in 0..length{
            index[i % (threads)] += 1;
        }
        for i in 0..threads.clone(){
            let mut tmp = 0;
            for j in 0..i+1{
                tmp += index[j]
            }
            indexes.push(tmp);
        }
        //println!("{:?}", indexes);
        //println!("{:?}", index);
        return indexes
    }
    
    //DOES NOT GUARANTEE ORDER IN RESULTING VECTOR
    //fn para_boot<F: (Fn(f64) -> f64) + Send + 'static + Copy>(&self, c: Vec<LWE>, func: F) -> Vec<LWE>{
    pub fn para_boot(&self, lwe_vec: Vec<LWE>, func: fn(f64) -> f64) -> Vec<LWE>{
        let (tx, rx) = mpsc::channel();
        let mut threads = vec![];
        let mut res_vec = vec![];
        
        let idx = self.slicing(lwe_vec.len().clone(), &self.max_threads);

        for i in 0..self.max_threads{
            let tx_clone = tx.clone();
            let enc_clone = self.enc.clone();
            let bsk_clone = self.bsk.clone();

            //let size = lwe_vec.len()/self.max_threads;
            let mut work: Vec<LWE> = lwe_vec[idx[i]..idx[i+1]].to_vec();

            let t = thread::spawn( move || { 
                for lwe_text in work.iter_mut(){
                    *lwe_text = lwe_text.bootstrap_with_function(&bsk_clone, |x| func(x), &enc_clone).unwrap();
                }
                tx_clone.send(work).unwrap();
            });
            threads.push(t);
        }
        for t in threads{
            t.join().unwrap();
        }
        while let Ok(results) = rx.try_recv(){
            res_vec.push(results);
        }
        return res_vec.into_iter().flatten().collect::<Vec<LWE>>();
        

    }
}
