use num_bigint::{BigInt, RandBigInt};
use rand::thread_rng;
use num_traits::{One, Zero};
use sha2::{Sha256, Digest};


const P: &str = "";
const G: u32 = 2;

fn mod(base:&BigInt, exp:&BigInt, modulus:&BigInt) -> BigInt {
    base.modpow(exp, modulus) // base^exp % modulus
}

fn hash_to_challenge(t:&BigInt) -> BigInt {
    let mut hasher = Sha256::new();
    hasher.update(t.to_bytes_be());
    BigInt::from_bytes_be(num_bigint::Sign::Plus, &hasher.finalize());
}

fn schnorr_prove(secret:&BigInt, p:&BigInt) -> (BigInt, BigInt) {
    let mut rng = thread_rng();
    let r = rng.gen_bigint_range(&BigInt::zero(), p); // generate random r 
    let t = mod_exp(&BigInt::from(G), &r, p);
    let c = hash_to_challenge(&t);
    let s = (r + c * secret) % p;
    (t, s)
}

fn schnorr_verify(t:&BigInt, s:&BigInt, public:&BigInt, p:&BigInt) -> bool {
    let lhs = mod_exp(&BigInt::from(G), s, p);
    let rhs = (t * mod_exp(public, &hash_to_challenge(t), p)) % p; // Computes t * h^c mod p 
    lhs == rhs // verifies g^s = t * h^c 
}

fn main() {
    let p = BigInt::parse_bytes(P.as_bytes(), 16).unwrap();
    let secret_value = BigInt::from(42); // Private key x 
    let public = mod_exp(&BigInt::from(G), &secret_value, &p); // Public key h = g^x mod p 
    
    let (t, s) = schnorr_prove(&secret_value, &p);

    println!("Prover: SENT (t={}, s={})", t, s);

    let valid = schnorr_verify(&t, &s, &public, &p);
    println!("Verifier: PROOF IS {}", valid);

}


