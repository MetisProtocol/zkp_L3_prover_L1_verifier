# zkp_L3_prover_L1_verifier
This repo will maintain proof generation and verifier codes written in rust and solidity

# Most of the installations work for linux machines and mac machines which support apt-get.
# Install following dependencies for C++

## sudo apt install g++
## sudo apt install libssl-dev
## sudo apt install libboost-all-dev
## sudo apt install libjsoncpp-dev
## sudo apt-get install libgtest-dev

# In mac : use this command for omp libraries and for g++ either install Xcode or the below gcc command using brew. 
## brew install libomp 
## brew install gcc48
## brew install openssl (Installation for ssl-dev)
## brew install boost (Installation for libboost)
## brew install jsoncpp (Installation for libjsoncpp)
## brew install googletest (Installation for googletest)

# Install Rust
## curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
## source $HOME/.cargo/env
## rustc --version


# Run Commands
## cd ~/zkp_L3_prover_L1_verifier/crypto/stark

## cargo test

## cargo run --example wrapper --arguments having values for  : 
   ### Assembly File
   ### Primary Tape
   ### Auxillary Tape
   ### Time Steps
   ### Security Parameter
   ### Prover
   ### Address
   ### Port Number
   ### Verbose
   ### Session
   ### Macros File



crypto/stark_verifier_ethereum - written in solidity code to verify proof.

crypto/stark - written in rust code to generate proof.

https://docs.google.com/document/d/1RrUBEJVJ-Ijf6x9RoIXPActFlZlxPVvq0jo3FTEpMDU/edit?pli=1#

