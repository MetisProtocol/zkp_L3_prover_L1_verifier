//#ifndef ZKMETIS_API_HPP
//#define ZKMETIS_API_HPP
//#endif
//#include <iostream>
#include <string>
#include <cstring> 
//#include "../
//#include "mips.hpp"
//#include <regex>
// #include "../zkMipsParser.hpp"
//#include "../executeProtocol.hpp"
//#include "protocols/protocol.hpp"
//#include "../RAMtoBair/ConstraintSystemToBair/cs2Bair.hpp"
//#include "../RAMtoBair/RamToContraintSystem/ALU.hpp"
//use std::ffi::CString;
//using namespace simple_mips::ACSP_FOR_MIPS;
// using namespace libstark;
#ifdef __cplusplus
extern "C" {
#endif
//template <libstark::BairWitness::permutation_t permutation_t>
struct BlobMetadata {
       int age;        
       char* witness;
    //   std::string witness;
   //    std::unique_ptr<cs2BairConstraints> witness;
     //  permutation_t;
     //  libstark::Sequence<permutation_t> permutation_ptr;
    };
    //  extern template struct BlobMetadata<libstark::BairWitness::permutation_t>;
   // void execute(const string assemblyFile, const string primaryTapeFile, const string auxTapeFile, const string& macros_file, const string& address, uint16_t port_number, const string& session, const size_t t, const size_t securityParameter, bool verbose);
     //  typedef BlobMetadata<libstark::BairWitness::permutation_t permutation_t> blogMeta;
       BlobMetadata execute(const size_t t, bool prover, uint16_t port_number, const char *assembly);
//     int execute(const size_t t, bool prover, uint16_t port_number, const char *assembly); 
   //  int  execute(const std::string assemblyFile, const std::string primaryTapeFile, const std::string auxTapeFile, const size_t t, bool prover, const std::string& address, uint16_t port_number, bool verbose, const std::string& session, const std::string& macros_file); 
#ifdef __cplusplus
}
#endif
