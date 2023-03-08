#include <iostream>
#include <fstream>
#include <string>
#include <languages/Bair/BairWitnessChecker.hpp>

#include <protocols/protocol.hpp>
#include "mips.hpp"
#include "mips_wrapper.hpp"

#include <sys/stat.h>
#include "executeProtocol.hpp"
#include "argParser.hpp"
#include "zkMipsParser.hpp"
#ifndef PRINT_HELPERS_HPP__
#include <protocols/print_helpers.hpp>
#endif

using namespace simple_mips;
using namespace simple_mips::ACSP_FOR_MIPS;

using std::cout;
using std::endl;
using std::string;
using std::stoul;

using std::vector;

int main(int argc, char *argv[]) {
  
   /* Parse arguments */
    if (argc < 2){
        print_help(argv[0], "No input asm file given.");
        return EXIT_SUCCESS;
    }
    ArgParser args(argc, argv);
    /* Print help message */
    if (args.cmd_option_exists("-h") || args.cmd_option_exists(help_msg_prefix)) {
        print_help(argv[0], "");
        return EXIT_SUCCESS;
    }
    /* Print usage examples */
    if (args.cmd_option_exists("-e") || args.cmd_option_exists(examples_prefix)) {
        print_examples(argv[0]);
        return EXIT_SUCCESS;
    }
    const string executable(argv[0]);
    const string zkmetis("zkmetis");
    const string path = executable.substr(0, executable.size() - zkmetis.size());
    /* Input zkMIPS file */
    string assemblyFile = args.get_cmd_option(zkmips_file_prefix);
    if (!args.cmd_option_exists(zkmips_file_prefix) || !file_exists(assemblyFile) ) {
        print_help(argv[0], "No input asm file given. Use the " + zkmips_file_prefix + " flag and then provide the asm file.");
        return EXIT_FAILURE;
    }
    zmips_filename_ = assemblyFile;
    string primaryTapeFile = "";
    if (args.cmd_option_exists(primary_tape_prefix)) {
        primaryTapeFile = args.get_cmd_option(primary_tape_prefix);
        if (!file_exists(primaryTapeFile)) {
            print_help(argv[0], primaryTapeFile + " does not exist. Use a valid public tape file.");
            return EXIT_FAILURE;
        }
    } else {
        string pathfile = getPathName(assemblyFile);
        string rawname = assemblyFile.substr(0, assemblyFile.find_last_of("."));
        if (file_exists(pathfile+"/pubtape.txt")) {
            primaryTapeFile = pathfile+"/pubtape.txt";
            std::cout << "No primary tape file is given, using " << primaryTapeFile << '\n';
        } else if (file_exists(rawname+".pubtape")) {
            primaryTapeFile = rawname+".pubtape";
            std::cout << "No primary tape file is given, using " << primaryTapeFile << '\n';
        }
    }
    string auxTapeFile = "";
    if (args.cmd_option_exists(private_tape_prefix)) {
        auxTapeFile = args.get_cmd_option(private_tape_prefix);
        if (!file_exists(auxTapeFile)) {
            print_help(argv[0], auxTapeFile + " does not exist. Use a valid private tape file.");
            return EXIT_FAILURE;
        }
    } else {
        string pathfile = getPathName(assemblyFile);
        string rawname = assemblyFile.substr(0, assemblyFile.find_last_of("."));
        if (file_exists(pathfile+"/auxtape.txt")) {
            auxTapeFile = pathfile+"/auxtape.txt";
            std::cout << "No private tape file is given, using " << auxTapeFile << '\n';
        } else if (file_exists(rawname+".auxtape")) {
            auxTapeFile = rawname+".auxtape";
            std::cout << "No private tape file is given, using " << auxTapeFile << '\n';
        }
    }
    /* Timesteps 2^t*/
    size_t executionLenLog = 5;
    bool tsteps_provided = false;
    if (args.cmd_option_exists(timesteps_prefix)) {
        executionLenLog = stol(args.get_cmd_option(timesteps_prefix));
        tsteps_provided = true;
    }
    /* soundness error at most 2^(-sec) */
    size_t securityParameter = 60;
    if (args.cmd_option_exists(security_prefix)) {
        securityParameter = stol(args.get_cmd_option(security_prefix));
    }
    bool no_proof = args.cmd_option_exists(no_proof_prefix);
    /* Run prover and verifier separately */
    bool prover = args.cmd_option_exists(run_prover_prefix);
    bool verifier = args.cmd_option_exists(run_verifier_prefix);
   
    if (prover == verifier && prover) {
        print_help(argv[0], "Cannot be both prover and verifier at the same time.");
        return EXIT_FAILURE;
    }
    if ((prover || verifier) && no_proof) {
        print_help(argv[0], "Cannot run the zMIPS execution engine remotely.");
        return EXIT_FAILURE;
    }
    bool verbose = args.cmd_option_exists(verbose_prefix);
    bool show_asm = args.cmd_option_exists(show_asm_prefix);
    bool debug = args.cmd_option_exists(debug_prefix);
    /* Parse address and port information */
    string address = "localhost";
    uint16_t port_number = 1234;
    if (args.cmd_option_exists(address_port_prefix)) {
        string arg_without_prefix = args.get_cmd_option(address_port_prefix);
        int pos = arg_without_prefix.find_first_of(':');
        address = arg_without_prefix.substr(0, pos);
        port_number = stoi(arg_without_prefix.substr(pos+1));
    }
    string session = "";
    if (args.cmd_option_exists(session_prefix)) {
        session = args.get_cmd_option(session_prefix);
    }
    // printHeader();
    /* assembly file can either be a ZK-MIPS file or a Zkmetis asm file */
    string asmFile = parse_zkmips(assemblyFile, primaryTapeFile, path+"framework/zkmetis/src/macros.json", show_asm);
    cout << "Test Prover " << "\n"; 
    if (prover) {
         cout << "Prover:\nExecuting over the network simulation with assembly from '" + assemblyFile + "' over 2^" + to_string(executionLenLog) +"-1 steps, soundness error at most 2^-" +to_string(securityParameter)+", public inputs from '" << primaryTapeFile <<"' and private inputs from '"+auxTapeFile<<"'. Verifier is at " << address << ":" << port_number<< ".\n\n";
        execute(asmFile, primaryTapeFile, auxTapeFile, executionLenLog, securityParameter, prover, address, port_number, verbose, session);
        cout << "Complete Prover \n";
    }
    if (!debug) std::remove(asmFile.c_str());
   
    return EXIT_SUCCESS;
   /* 
   if(argc != 3) {
        cout << "please pass 2 arguments." << endl;
        return 0;
    }

    const unsigned int a_num(stoul(argv[1]));
    const unsigned int b_num(stoul(argv[2]));
    unsigned int securityParameter = 60
    return 0;
    */
}
