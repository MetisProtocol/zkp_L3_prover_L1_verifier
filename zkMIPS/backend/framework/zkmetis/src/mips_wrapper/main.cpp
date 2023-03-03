#include <iostream>
#include <fstream>
#include <string>
#include <languages/Bair/BairWitnessChecker.hpp>

#include <protocols/protocol.hpp>
#include "mips.hpp"
#include "mips_wrapper.hpp"

using namespace simple_mips;
using namespace simple_mips::ACSP_FOR_MIPS;

using std::cout;
using std::endl;
using std::string;
using std::stoul;

using std::vector;

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
