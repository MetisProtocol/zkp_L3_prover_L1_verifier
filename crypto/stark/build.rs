#![allow(non_snake_case)]

// extern crate glob;
// extern crate cc;
// extern crate bindgen;

use glob::glob;
use std::env;
use std::path::PathBuf;

fn main() {    
    let stark = glob("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/libstark/src/**/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();
    let algebralib = glob("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/algebralib/**/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();
    let FFT = glob("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/FFT/**/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();    
    let gadgetlib = glob("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/framework/gadgetlib/gadgetlib/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();
    let mips = glob("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/framework/zkmetis/src/**/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();    
    print!(" Test Stark !");    
    cc::Build::new()
        .cpp(true)                
        .flag("-xc++")
        .flag("-std=c++11")
     //   .flag("-O3")
    //    .flag("-g")
   //     .flag("-w")
  //      .flag("-Wall")
        .flag("-fmessage-length=0")        
        .flag("-openmp")
        .flag("-maes")
        .flag("-msse4")
//        .flag("-mtune=native")
//        .flag("-Isrc")   
//        .flag("-Xpreprocessor")
//        .flag("-pthread")   
 //       .flag("-lomp")              
        .static_flag(true)
        .warnings(false)
        .extra_warnings(false)
        .files(stark)        
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/algebralib/headers")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/FFT/src")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/libstark/src")    
        .compile("stark");

    print!(" Hello stark!");    
    cc::Build::new()
        .cpp(true)    
        .flag("-xc++")
        .flag("-std=c++11")
    //    .flag("-O3")
    //    .flag("-g")
    //    .flag("-w")
    //    .flag("-Wall")
     //   .flag("-fmessage-length=0")        
        .flag("-openmp")
        .flag("-maes")
        .flag("-msse4")
    //    .flag("-mtune=native")
    //    .flag("-Isrc")   
    //    .flag("-Xpreprocessor")
  //      .flag("-pthread")   
  //      .flag("-lomp")  
        .static_flag(true)
        .warnings(false)
        .extra_warnings(false)
        .files(algebralib)                
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/algebralib/headers")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/FFT/src")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/libstark/src")    
        .compile("algebralib");

    print!(" Hello Stark 1!");

    cc::Build::new()
        .cpp(true)    
        .flag("-xc++")
        .flag("-std=c++11")
     //   .flag("-O3")
    //    .flag("-g")
    //    .flag("-w")
     //   .flag("-Wall")
      //  .flag("-fmessage-length=0")        
        .flag("-openmp")
        .flag("-maes")
        .flag("-msse4")
     //   .flag("-mtune=native")
    //    .flag("-Isrc")   
    //    .flag("-Xpreprocessor")
    //    .flag("-pthread")   
    //    .flag("-lomp")  
        .flag("-mpclmul") 
        .static_flag(true)
        .warnings(false)
        .extra_warnings(false)
        .files(FFT)        
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/algebralib/headers")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/FFT/src")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/libstark/src")
        .compile("FFT");

    print!(" Hello Stark 2!");

   cc::Build::new()
        .cpp(true)    
        .flag("-xc++")
        .flag("-std=c++11")
//        .flag("-O3")
//        .flag("-g")
//        .flag("-w")
//        .flag("-Wall")
//        .flag("-fmessage-length=0")        
        .flag("-openmp")
        .flag("-maes")
        .flag("-msse4")
 //       .flag("-mtune=native")
 //       .flag("-Isrc")   
 //       .flag("-Xpreprocessor")
 //       .flag("-pthread")   
 //       .flag("-lomp")  
 //       .flag("-mpclmul") 
        .static_flag(true)
        .warnings(false)
        .extra_warnings(false)
        .files(gadgetlib)
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/framework/gadgetlib")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/algebralib/headers")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/FFT")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/FFT/..")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/FFT/src")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/libstark/src")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/framework/gadgetlib/gadgetlib")  
        .compile("gadgetlib");

    print!(" Hello Stark 4!");

    cc::Build::new()
        .cpp(true)        
        .flag("-xc++")
        .flag("-std=c++11")
    //    .flag("-O3")
    //    .flag("-g")
   //    .flag("-w")
 //       .flag("-Wall")
  //      .flag("-fmessage-length=0")        
        .flag("-openmp")
        .flag("-maes")
        .flag("-msse4")
   //     .flag("-mtune=native")
    //    .flag("-Isrc")   
   //     .flag("-Xpreprocessor")
   //     .flag("-pthread")   
   //     .flag("-lomp")                      
        .static_flag(true)     
        .warnings(false)
        .extra_warnings(false) 
        .files(mips)        
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/algebralib/headers")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/algebra/FFT/src")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/libstark/src")
        .include("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/framework/gadgetlib")
        .compile("mips");

    println!(" Hello Stark 3");     

    let bindings = bindgen::builder()        
        .header("/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/framework/zkmetis/src/mips_wrapper/mips_wrapper.hpp")               
        .trust_clang_mangling(false)
        .rustfmt_bindings(true)      
        .enable_cxx_namespaces()
        .clang_arg(r"-xc++")
        .clang_arg(r"-std=c++11")
        .clang_arg("-Isrc")
        .clang_arg("-Ihome/ubuntu/zkMIPS/backend/algebra/algebralib/headers")    
        .clang_arg("-Ihome/ubuntu/zkMIPS/backend/algebra/FFT/src")
        .clang_arg("-Ihome/ubuntu/zkMIPS/backend/libstark/src")
        .whitelist_function("execute")          
        .derive_copy(false)
        .layout_tests(false)     
        .generate()
        .expect("Unable to generte bindings");

    let out_path = PathBuf::from("/home/ubuntu/zkp_L3_prover_L1_verifier/crypto/stark/src");
    
    bindings
        .write_to_file(out_path.join("bindings.rs"))        
        .expect("Couldn't write bindings!");    
    
}
