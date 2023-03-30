use structopt::StructOpt;
use ::std::os::raw::c_uint;
use ::std::os::raw::c_schar;
use ::std::os::raw::c_int;
use ::std::os::raw::c_short;
use std::convert::TryInto;
//use zkp_stark::root::std::string;
//use cxx::CxxString;
// use std::string;
#[derive(StructOpt)]
struct Opt {
//    #[structopt(name = "Assembly File")]
//    assemblyFile: string,
//    #[structopt(name = "Primary Tape")]
//    primaryTapeFile: string,
 //   #[structopt(name = "Auxillary Tape")]
 //   auxTapeFile: string,
    #[structopt(name = "Time Steps")]
    t: u64,
//    #[structopt(name = "Security Parameter")]
//    securityParameter: string,
    #[structopt(name = "Prover", parse(try_from_str))]
    prover: bool,
//    #[structopt(name = "Address")]
//    address: string,
    #[structopt(name = "Port Number")]
    port_number: c_uint,
//    #[structopt(name = "Verbose", parse(try_from_str))]
//    verbose: bool,
//    #[structopt(name = "Session")]
//    session: string,
 //   #[structopt(name = "Macros File")]
 //   macros_file: string
    
}


fn main() {
    let mut opt = Opt::from_args();
    opt.t = 5;
    opt.prover = true;
    opt.port_number = 8081; 
    let security_parameter: c_int = 60;    
    println!("Test Param 1");
    zkp_stark::execute(opt.t, opt.prover.try_into().unwrap(), opt.port_number.try_into().unwrap());    
    println!("Test Param");
}
