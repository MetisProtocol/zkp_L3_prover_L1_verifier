#include <iostream>
#include <fstream>
#include <string>
#include <languages/Bair/BairWitnessChecker.hpp>

#include <protocols/protocol.hpp>
#include "Add.hpp"

using namespace simple_add;
using namespace simple_add::ACSP_FOR_ADD;

using std::cout;
using std::endl;
using std::string;
using std::stoul;

using std::vector;

// a, b: secret numbers of the initial values of a fibonacci sequence for some sequence length
void execute(const unsigned a, const unsigned int b, const unsigned securityParameter) {
    
    libstark::BairInstance bair_instance = buildBairInstance(a, b);
    Add::evalp::setParams(1234);
    libstark::BairWitness bair_witness = buildBairWitness(a, b);

    std::cout << "verify:" << libstark::BairWitnessChecker::verify(bair_instance, bair_witness) << std::endl;   
    libstark::Protocols::executeProtocol(bair_instance, bair_witness, securityParameter, false, false, true);
}

int main(int argc, char *argv[]) {
    if(argc != 3) {
        cout << "please pass 2 arguments." << endl;
        return 0;
    }

    const unsigned int a_num(stoul(argv[1]));
    const unsigned int b_num(stoul(argv[2]));
    unsigned int securityParameter = 60;

    execute(a_num, b_num, securityParameter);
    return 0;
}