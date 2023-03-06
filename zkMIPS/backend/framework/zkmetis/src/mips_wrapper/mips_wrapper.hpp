#ifndef ZKMETIS_API_HPP
#define ZKMETIS_API_HPP

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

    void execute(const unsigned int a, const unsigned int b, const unsigned int securityParameter);

#ifdef __cplusplus
}
#endif
