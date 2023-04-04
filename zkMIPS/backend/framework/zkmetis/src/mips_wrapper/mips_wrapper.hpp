//#ifndef ZKMETIS_API_HPP
//#define ZKMETIS_API_HPP
//#endif
//#include <iostream>
#include <string>
#include <cstring>
//#include <regex>
//#include "../zkMipsParser.hpp"
//#include "../executeProtocol.hpp"
//#include "protocols/protocol.hpp"
//#include "../RAMtoBair/ConstraintSystemToBair/cs2Bair.hpp"
//#include "../RAMtoBair/RamToContraintSystem/ALU.hpp"
//use std::ffi::CString;
#ifdef __cplusplus
extern "C" {
#endif

   // void execute(const string assemblyFile, const string primaryTapeFile, const string auxTapeFile, const string& macros_file, const string& address, uint16_t port_number, const string& session, const size_t t, const size_t securityParameter, bool verbose);

    void  execute(const std::string assemblyFile, const std::string primaryTapeFile, const std::string auxTapeFile, const size_t t, bool prover, const std::string& address, uint16_t port_number, bool verbose, const std::string& session, const std::string& macros_file); 
#ifdef __cplusplus
}
#endif
