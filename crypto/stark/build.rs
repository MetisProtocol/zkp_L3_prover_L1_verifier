#![allow(non_snake_case)]

// extern crate glob;
// extern crate cc;
// extern crate bindgen;

use glob::glob;
use std::env;
use std::path::PathBuf;

fn main() {    
    let stark = glob("backend/libstark/src/**/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();
    let algebralib = glob("backend/algebra/algebralib/**/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();
    let FFT = glob("backend/algebra/FFT/**/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();    
    let gadgetlib = glob("backend/framework/gadgetlib/gadgetlib/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();
    let mips = glob("backend/framework/zkmetis/src/**/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();    
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
        .include("backend/algebra/algebralib/headers")
        .include("backend/algebra/FFT/src")
        .include("backend/libstark/src")    
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
        .include("backend/algebra/algebralib/headers")
        .include("backend/algebra/FFT/src")
        .include("backend/libstark/src")    
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
        .include("backend/algebra/algebralib/headers")
        .include("backend/algebra/FFT/src")
       .include("backend/libstark/src")
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
        .include("backend/framework/gadgetlib")
        .include("backend/algebra/algebralib/headers")
        .include("backend/algebra/FFT")
        .include("backend/algebra/FFT/..")
        .include("backend/algebra/FFT/src")
        .include("backend/libstark/src")
        .include("backend/framework/gadgetlib/gadgetlib")  
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
        .include("backend/algebra/algebralib/headers")
        .include("backend/algebra/FFT/src")
        .include("backend/libstark/src")
        .include("backend/framework/gadgetlib")
        .compile("mips");

    println!(" Hello Stark 3");     
    println!("cargo:rustc-link-arg=-fopenmp");
    println!("cargo:rustc-link-lib=gomp");
    println!("cargo:rustc-link-lib=jsoncpp");
    let out_dir = "/home/ubuntu/zkp_L3_prover_L1_verifier/crypto/stark";
    println!("cargo:rustc-link-search=native={}", out_dir); 
 //  println!("cargo:rustc-link-lib=liblibstark.a");
  //  println!("cargo:rustc-link-search=/home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/bin/libstark"); 
 //  println!("cargo:rustc-flags="-L /home/ubuntu/zkp_L3_prover_L1_verifier/zkMIPS/backend/libstark/src"");
    
    let bindings = bindgen::builder()        
        .header("backend/framework/zkmetis/src/mips_wrapper/mips_wrapper.hpp")               
        .trust_clang_mangling(false)
        .rustfmt_bindings(true)      
        .enable_cxx_namespaces()
        .clang_arg(r"-xc++")
        .clang_arg(r"-std=c++11")
        .clang_arg("-Isrc")
        .clang_arg("-Ibackend/algebra/algebralib/headers/")    
        .clang_arg("-Ibackend/algebra/FFT/src/")
        .clang_arg("-Ibackend/libstark/src/")
        .whitelist_function("execute")          
        .derive_copy(false)
        .layout_tests(false)     
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("/home/ubuntu/zkp_L3_prover_L1_verifier/crypto/stark/src");
    
    bindings
        .write_to_file(out_path.join("bindings.rs"))        
        .expect("Couldn't write bindings!");    
    
}
