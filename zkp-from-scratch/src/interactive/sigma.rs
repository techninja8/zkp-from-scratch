use curv_kzen::elliptic::curves::{Secp256k1, ECScalar, ECPoint};
use rand::Rng;

// Define the Schnorr Sigma Protocol
struct SigmaProtocol {
    g: ECPoint<Secp256k1>, // Generator
    x: ECPoint<Secp256k1>, // Public key (g^w)
    w: ECScalar<Secp256k1>, // Secret (private key)
}

impl SigmaProtocol {
    // Initialize with a random secret w
    fn new() -> Self {
        let w = ECScalar::<Secp256k1>::new_random();
        let g = ECPoint::<Secp256k1>::generator();
        let x = g * &w;
        Self { g, x, w }
    }

    // Prover: Generate commitment
    fn commit(&self) -> (ECPoint<Secp256k1>, ECScalar<Secp256k1>) {
        let r = ECScalar::<Secp256k1>::new_random();
        let t = self.g * &r; // t = g^r
        (t, r)
    }

    // Verifier: Generate random challenge
    fn challenge() -> ECScalar<Secp256k1> {
        let mut rng = rand::thread_rng();
        let e = rng.gen::<u64>(); // Random challenge
        ECScalar::<Secp256k1>::from(&e)
    }

    // Prover: Generate response z
    fn response(&self, r: &ECScalar<Secp256k1>, e: &ECScalar<Secp256k1>) -> ECScalar<Secp256k1> {
        r + e * &self.w // z = r + ew
    }

    // Verifier: Validate proof
    fn verify(&self, t: &ECPoint<Secp256k1>, e: &ECScalar<Secp256k1>, z: &ECScalar<Secp256k1>) -> bool {
        let lhs = self.g * z; // g^z
        let rhs = t + self.x * e; // t * x^e
        lhs == rhs
    }
}

// Main function to run the Sigma Protocol
fn main() {
    let sigma = SigmaProtocol::new();

    // Step 1: Prover commits
    let (t, r) = sigma.commit();
    println!("Commitment (t): {:?}", t);

    // Step 2: Verifier challenges
    let e = SigmaProtocol::challenge();
    println!("Challenge (e): {:?}", e);

    // Step 3: Prover responds
    let z = sigma.response(&r, &e);
    println!("Response (z): {:?}", z);

    // Step 4: Verifier verifies
    let valid = sigma.verify(&t, &e, &z);
    println!("Verification: {}", valid);
}
