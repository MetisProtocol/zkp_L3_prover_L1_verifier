#ifndef __ACSP_FOR_MIPS_HPP__
#define __ACSP_FOR_MIPS_HPP__

#include <iostream>
#include <string>
#include <regex>
#include "../zkMipsParser.hpp"
#include "../executeProtocol.hpp"
#include "protocols/protocol.hpp"
#include "../RAMtoBair/ConstraintSystemToBair/cs2Bair.hpp"
#include "../RAMtoBair/RamToContraintSystem/ALU.hpp"
// #include <languages/Bair/BairInstance.hpp>
// #include <languages/Bair/BairWitness.hpp>

#include <algebraLib/BitExtract.hpp>

#define PRNMSG(str) do { std::cout << str << std::endl; } while( false )
#define DBGMSG(str) do { } while ( false )

#define EXTDIM 64 

// using namespace libstark::languages::Bair;
using namespace Algebra;

using std::cout;
using std::endl;
using std::string;

namespace mips {

    // FieldElement evalCPoly(const std::vector<FieldElement> & vars,
	// 	const std::vector<FieldElement> & RootHash, const FieldElement& gN);

    class evalp {
        public:
            static void setParams(const int);
            static FieldElement ep(const std::vector<FieldElement>&);                   
    };

    const unsigned long lastStep = 3;
    const short NUMREGS = 3;     

    namespace reg {
        typedef enum RegType
        {
            A = 0,
            B,
            C        
        } RegType;
    }

    typedef std::vector<std::vector<FieldElement>> & witnessType;
    short getDim(long long);
    void genWitnessAddWithPadding(witnessType arr, const unsigned int, const unsigned int);
    
}


namespace simple_mips{
    namespace ACSP_FOR_MIPS{        
        libstark::BairInstance buildBairInstance(const unsigned int, const unsigned int);
        libstark::BairWitness buildBairWitness(const unsigned int, const unsigned int);
    }
}

#endif // __ACSP_FOR_MIPS_HPP__
