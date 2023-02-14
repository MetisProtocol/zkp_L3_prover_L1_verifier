#include "Add.hpp"
#include "common/Utils/ErrorHandling.hpp"

namespace simple_add{
namespace ACSP_FOR_ADD{

    using std::vector;
    using Algebra::FieldElement;
    using namespace Add;

    using namespace Algebra;

    namespace{        
        class idPermutation : public libstark::BairWitness::permutation_t{
            public:
                size_t getElementByIndex(index_t index)const {
                    return index;
                }
        };

        class coloringClass : public libstark::BairWitness::assignment_t{
            private:
                std::vector<libstark::BairWitness::color_t> coloring_;                
            public:
                coloringClass(const unsigned int a, const unsigned int b):
                    // coloring_((size_t(1) << Add::getDim(commonParams.length)) - 1, vector<FieldElement>(Add::NUMREGS))              
                    coloring_(Add::lastStep + 1 , vector<FieldElement>(Add::NUMREGS)) 
                {                                                             
                    Add::genWitnessAddWithPadding(coloring_, a, b);                    
                }

                libstark::BairWitness::color_t getElementByIndex(index_t index)const{
                    _COMMON_ASSERT(index < coloring_.size(), "index of color out of range");
                    return coloring_[index];
                }
        };
    }

    libstark::BairWitness buildBairWitness(const unsigned int a, const unsigned int b){
        using libstark::BairWitness;
        BairWitness::permutation_ptr perm(new idPermutation());
        BairWitness::assignment_ptr assignment(new coloringClass(a, b));

        return BairWitness(std::move(assignment), std::move(perm));
    }
}    
}