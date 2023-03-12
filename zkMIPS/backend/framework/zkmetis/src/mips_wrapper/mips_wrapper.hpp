#ifndef ZKMETIS_API_HPP
#define ZKMETIS_API_HPP
#endif
#include <iostream>
#include <string>
#include <regex>
#include "zkMipsParser.hpp"
#include "executeProtocol.hpp"
#include "protocols/protocol.hpp"
#include "RAMtoBair/ConstraintSystemToBair/cs2Bair.hpp"
#include "RAMtoBair/RamToContraintSystem/ALU.hpp"

#ifdef __cplusplus
extern "C" {
#endif

   // void execute(const string assemblyFile, const string primaryTapeFile, const string auxTapeFile, const string& macros_file, const string& address, uint16_t port_number, const string& session, const size_t t, const size_t securityParameter, bool verbose);

    void  execute(const string assemblyFile, const string primaryTapeFile, const string auxTapeFile, const size_t t, const size_t securityParameter, bool prover, const string& address, uint16_t port_number, bool verbose, const string& session, const string& macros_file); 
#ifdef __cplusplus
}
#endif
