#include "Add.hpp"

#define RI(n) n
#define RN 4

using namespace Algebra;

namespace Add{

    FieldElement randCoeff[RN];
    
    void evalp::setParams(const int prngseed) {
        Algebra::rng.seed(prngseed);        
        for (int i = 0; i < RN; i++)
            randCoeff[i] = Algebra::generateRandom(); 
    }

    FieldElement evalp::ep(const std::vector<FieldElement>& vars) {         

        FieldElement tval = randCoeff[RI(0)] * (vars[reg::A + NUMREGS] + vars[reg::B]);
        tval += randCoeff[RI(1)] * (vars[reg::B + NUMREGS] + vars[reg::C]);
        tval += randCoeff[RI(2)] * (vars[reg::C + NUMREGS] + vars[reg::A]);
        tval += randCoeff[RI(3)] * (vars[reg::A] + vars[reg::B] + vars[reg::C]);

        return tval;
    }

} //namespace