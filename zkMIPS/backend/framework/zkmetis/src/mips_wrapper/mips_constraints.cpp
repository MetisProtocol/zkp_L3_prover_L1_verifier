#include "mips_instance.hpp"
#include <algebraLib/BitExtract.hpp>
//#include "languages/Bair/BairInstance.hpp"
//#include "languages/Bair/BairWitness.hpp"
#include "mips.hpp"
#define ttgenRand (Algebra::one()) 

using Algebra::degreeOfProduct;
using Algebra::FieldElement;
using Algebra::generateRandom;
using Algebra::one;
using Algebra::PolynomialDegree;
using Algebra::PolynomialInterface;
using namespace libstark; 

namespace simple_mips {
namespace ACSP_FOR_MIPS{

namespace { // anonymous namespace for polys and common vars
    using Algebra::mapIntegerToFieldElement;

class polyAdd_class : public PolynomialInterface {
    public:
        polyAdd_class() {};

        Algebra::FieldElement eval(const std::vector<FieldElement>& x)const{
            return mips::evalp::ep(x);
        }

        bool isEffectiveInput(const size_t varId)const{
            switch(varId)
            {                                
                default: return true;
            }
        }

        Algebra::PolynomialDegree getDegreeBound(const std::vector<PolynomialDegree>& inputDegrees)const{

            struct ttdeg{
                long long deg_;
                ttdeg(const PolynomialDegree& d):deg_(PolynomialDegree::integral_t(d)){};
                ttdeg(const long long& d):deg_(d){};

                ttdeg operator*(const ttdeg& d)const{
                    return degreeOfProduct(PolynomialDegree(deg_),PolynomialDegree(d.deg_));
                }
                
                ttdeg operator+(const ttdeg& d)const{
                    return std::max(deg_,d.deg_);
                }
            };

            const ttdeg A = ttdeg(inputDegrees[mips::reg::A]);          
            const ttdeg A_next = ttdeg(inputDegrees[mips::reg::A + mips::NUMREGS]);
            const ttdeg B = ttdeg(inputDegrees[mips::reg::B]);
            const ttdeg B_next = ttdeg(inputDegrees[mips::reg::B + mips::NUMREGS]);
            const ttdeg C = ttdeg(inputDegrees[mips::reg::C]);
            const ttdeg C_next = ttdeg(inputDegrees[mips::reg::C + mips::NUMREGS]);

            const ttdeg resTmp = (A + B + C) + (A_next + A) + (B_next + B) + (C_next + C);

            return PolynomialDegree(resTmp.deg_);
        }

        std::unique_ptr<PolynomialInterface> clone()const{
            return std::unique_ptr<PolynomialInterface>(new polyAdd_class);
        }

        size_t numVars()const{
            return mips::NUMREGS;
        }
};
} 

    Mips_CS::Mips_CS() {
        polys_.push_back(polyPtr_t(new polyAdd_class()));
    }

    Mips_CS* Mips_CS::clone() const{
        return new Mips_CS();
    }

    using std::vector;
    vector<FieldElement> Mips_CS::eval(const vector<FieldElement>& assignment) const{
        vector<FieldElement> res;
        for(const auto& p: polys_){
            res.push_back(p->eval(assignment));
        }
        return res;
    }

    } // ACSP_FOR_MIPS namespace
} // simple_mips namespace
