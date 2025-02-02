#![allow(deprecated)]
#![allow(unused_imports)]

use num_integer::Integer;
use num_bigint::{BigInt, RandBigInt};
use rand::thread_rng;
use num_traits::{One, Zero};
use sha2::{Sha256, Digest};


const P: &str = "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD1";
const G: u32 = 2;

fn mod_exp(base:&BigInt, exp:&BigInt, modulus:&BigInt) -> BigInt {
    base.modpow(exp, modulus) // base^exp % modulus
}

fn hash_to_challenge(t:&BigInt) -> BigInt {
    let mut hasher = Sha256::new();
    hasher.update(t.to_bytes_be().1);
    let hash_result = hasher.finalize();
    BigInt::from_bytes_be(num_bigint::Sign::Plus, &hash_result)
}

fn schnorr_prove(secret:&BigInt, p:&BigInt) -> (BigInt, BigInt, BigInt) {
    let mut rng = thread_rng();
    let r = rng.gen_bigint_range(&BigInt::zero(), p); // generate random r 
    let t = mod_exp(&BigInt::from(G), &r, p);
    let c = hash_to_challenge(&t);
    let s = (r + c.clone() * secret).mod_floor(p); // use mod_floor to ensure non-negative results
    (t, s, c)
}

fn schnorr_verify(t:&BigInt, s:&BigInt, public:&BigInt, p:&BigInt, c:&BigInt) -> bool {
    let lhs = mod_exp(&BigInt::from(G), s, p);
    // let rhs = (t * mod_exp(public, &hash_to_challenge(t), p)) % p; // Computes t * h^c mod p 
    let rhs = (t * mod_exp(public, c, p)) % p;
    lhs == rhs // verifies g^s = t * h^c 
}

fn main() {
    let p = BigInt::parse_bytes(P.as_bytes(), 16).unwrap();
    let secret_value = BigInt::from(42); // Private key x 
    let public = mod_exp(&BigInt::from(G), &secret_value, &p); // Public key h = g^x mod p 
    
    let (t, s, c) = schnorr_prove(&secret_value, &p);

    println!("Prover: SENT (t={}, s={})", t, s);

    let valid = schnorr_verify(&t, &s, &public, &p, &c);
    println!("Verifier: PROOF IS {}", valid);

}


