#include <iostream>
#include <fstream>
#include <string>
#include <languages/Bair/BairWitnessChecker.hpp>

#include <protocols/protocol.hpp>
#include "mips.hpp"
#include "mips_wrapper.hpp"

using namespace simple_fsrs;
using namespace simple_fsrs::ACSP_FOR_FSRS;

using std::cout;
using std::endl;
using std::stoul;
using std::string;

using std::vector;

// a, b: secret numbers of the initial values of a fibonacci sequence for some sequence length
void execute(const unsigned int a, const unsigned int b, const unsigned int securityParameter) {
    
    libstark::BairInstance bair_instance = buildBairInstance(a, b);
    Fsrs::evalp::setParams(1234);
    libstark::BairWitness bair_witness = buildBairWitness(a, b);

    std::cout << "verify:" << libstark::BairWitnessChecker::verify(bair_instance, bair_witness) << std::endl;   
    libstark::Protocols::executeProtocol(bair_instance, bair_witness, securityParameter, false, false, true);
}
