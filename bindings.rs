/* automatically generated by rust-bindgen */

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    extern "C" {
        pub fn execute(
            assemblyFile: ::std::os::raw::c_schar,
            primaryTapeFile: ::std::os::raw::c_schar,
            auxTapeFile: ::std::os::raw::c_schar,
            t: ::std::os::raw::c_schar::c_int,
            securityParameter: ::std::os::raw::c_schar,
            prover: ::std::os::raw::c_int,
            address: ::std::os::raw::c_schar,
            port_number: ::std::os::raw::c_uint,
            verbose: ::std::os::raw::c_int,
            session: ::std::os::raw::c_schar,
            macros_file: ::std::os::raw::c_schar  
        );
    }
}
