use structopt::StructOpt;
use ::std::os::raw::c_uint;
use ::std::os::raw::c_schar;
use ::std::os::raw::c_int;
use ::std::os::raw::c_short;
#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "Assembly File")]
    assemblyFile: c_schar,
    #[structopt(name = "Primary Tape")]
    primaryTapeFile: c_schar,
    #[structopt(name = "Auxillary Tape")]
    auxTapeFile: c_schar,
    #[structopt(name = "Time Steps")]
    t: u64,
    #[structopt(name = "Security Parameter")]
    securityParameter: c_schar,
    #[structopt(name = "Prover")]
    prover: bool,
    #[structopt(name = "Address")]
    address: c_schar,
    #[structopt(name = "Port Number")]
    port_number: c_uint,
    #[structopt(name = "Verbose")]
    verbose: c_int,
    #[structopt(name = "Session")]
    session: c_schar,
    #[structopt(name = "Macros File")]
    macros_file: c_schar
    
}


fn main() {
    let opt = Opt::from_args();
    let security_parameter: c_int = 60;    
    zkp_stark::execute(opt.assemblyFile, opt.primaryTapeFile, opt.auxTapeFile, opt.t, opt.prover, opt.address, opt.port_number, opt.verbose, opt.session, opt.macros_file);    
}
