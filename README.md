# Zero-Knowledge Proofs from Scratch in Rust

Pls note, this project is still under development.

## Overview
This repository contains a **from-scratch** implementation of various Zero-Knowledge Proof (ZKP) systems in Rust, covering:
- **Interactive Proofs** (e.g., Schnorr, Sigma Protocols)
- **Non-Interactive Proofs** (Fiat-Shamir, zk-SNARKs, zk-STARKs)
- **Core Mathematical Foundations** (Finite Fields, Elliptic Curves, FFT, R1CS, QAP)

The goal is to learn these key components of ZKP by **deconstructing** and **rebuildig** as much as I possibly can,while also ensuring simplicity and effieciency

## Features
✅ **Modular Rust Implementation**: Each proof system is implemented in a structured and extensible manner.  
✅ **Mathematical Rigor**: Detailed implementations of algebraic structures used in zk-proofs.  
✅ **Practical Demonstrations**: Example use cases for each proof system.  
✅ **No External zk Libraries**: This is a from-scratch implementation without reliance on zk-friendly Rust libraries.  

## Project Structure

For now, this is pretty much how the project structure would look like, I'll be updating it as it increase
```
zkp-from-scratch/
│── src/                          # Source code
│   ├── lib.rs                    # Main library file
│   ├── interactive/               # Interactive ZKPs (e.g., Schnorr)
│   │   ├── schnorr.rs             # Schnorr Protocol Implementation
│   │   ├── sigma.rs               # Sigma Protocols
│── tests/                         # Unit and integration tests
│── Cargo.toml                     # Rust dependencies and metadata
│── README.md                      # Project documentation
│── .gitignore                      # Ignore compiled binaries and target/

```

## Installation
Ensure you have **Rust** installed:
```sh
rustup update
```
Clone the repository and build the project:
```sh
git clone https://github.com/yourusername/zkp-from-scratch.git
cd zkp-from-scratch
cargo build --release
```

## Usage
Run an example proof system, such as Schnorr’s interactive proof:
```sh
cargo run --example schnorr_demo
```

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

## License
This project is licensed under the MIT License.


