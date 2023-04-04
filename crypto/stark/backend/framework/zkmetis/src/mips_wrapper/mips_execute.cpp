#include <iostream>
#include <fstream>
#include <string>
// #include <languages/Bair/BairWitnessChecker.hpp>
#include "../zkmetis-api.hpp"
#include <protocols/protocol.hpp>
#include "mips.hpp"
#include "mips_wrapper.hpp"
#include <sys/stat.h>

using namespace simple_mips;
using namespace simple_mips::ACSP_FOR_MIPS;
using namespace libstark;

using std::cout;
using std::endl;
using std::stoul;
using std::string;

using std::vector;
using namespace std;

inline bool file_exists(const string& name) {
    struct stat buffer;
    return (stat (name.c_str(), &buffer) == 0);
}

// a, b: secret numbers of the initial values of a fibonacci sequence for some sequence length
//void execute(const string assemblyFile, const string primaryTapeFile, const string auxTapeFile, const size_t t, bool prover, const string& address, uint16_t port_number, bool verbose, const string& session, const string& macros_file) 
int execute(const size_t t, bool prover, uint16_t port_number) {
    const string assemblyFile = "examples/add.mips";// "examples/read_test/read_test.mips";
    const string primaryTapeFile = "";// "examples/read_test/read_test.pubtape";
    const string auxTapeFile = "";//"examples/read_test/read_test.auxtape";
    const string& address = "localhost";
    bool verbose = true;
    const string& session = "10";
    const string& macros_file = "backend/framework/zkmetis/src/macros.json"; 
    std::cerr << " Test does not exist.\n";
    if (primaryTapeFile != "" && !file_exists(primaryTapeFile)) {
        std::cerr << "File " << primaryTapeFile << " does not exist.\n";
        
        exit(EXIT_FAILURE);
    }
    if (auxTapeFile != "" && !file_exists(auxTapeFile)) {
        std::cerr << "File " << auxTapeFile << " does not exist.\n";
        exit(EXIT_FAILURE);
    }
   // string macros_file = "";
    string asmFile = parse_zkmips(assemblyFile, primaryTapeFile, macros_file, false);
    //Initialize instance
    initRAMParamsFromEnvVariables();
    RAMProgram program(asmFile, REGISTERS_NUMBER, trRegisterLen);
    program.addInstructionsFromFile(asmFile);
    std::remove(asmFile.c_str());
    const auto bairInstance = constructInstance(program, t);
    // Read private inputs (auxTapeFile) to private_lines vector
    regex regex{R"([\n]+)"}; // split to lines
    ifstream auxtapefs(auxTapeFile);
    string private_inputs((std::istreambuf_iterator<char>(auxtapefs)),std::istreambuf_iterator<char>());
    sregex_token_iterator pr_it{private_inputs.begin(), private_inputs.end(), regex, -1};
    vector<string> private_lines{pr_it, {}};
    const auto bairWitness = constructWitness(program, t, private_lines);     // witness is generated from the prover
    if (!found_answer_) {
        std::cout << "\nTried for 2^15-1 timesteps and did not find answer.\n";
        return 1;
    }
//    libstark::Protocols::executeProverProtocol(bairInstance, bairWitness, address, port_number, verbose, answer_, session);
    libstark::Protocols::executeProtocol(bairInstance, bairWitness, 120, false, false, true, verbose);  
    std::cout << "\n Test Execute.\n";
    return 1;
    
 /*    
    libstark::BairInstance bair_instance = buildBairInstance(a, b);
    mips::evalp::setParams(1234);
    libstark::BairWitness bair_witness = buildBairWitness(a, b);

    std::cout << "verify:" << libstark::BairWitnessChecker::verify(bair_instance, bair_witness) << std::endl;   
    libstark::Protocols::executeProtocol(bair_instance, bair_witness, securityParameter, false, false, true);
  */
}
