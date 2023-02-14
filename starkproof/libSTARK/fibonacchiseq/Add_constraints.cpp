#include "Add_instance.hpp"
#include <algebraLib/BitExtract.hpp>
#include "languages/Bair/BairInstance.hpp"
#include "languages/Bair/BairWitness.hpp"
#include "Add.hpp"
#define ttgenRand (Algebra::one()) 

using Algebra::degreeOfProduct;
using Algebra::FieldElement;
using Algebra::generateRandom;
using Algebra::one;
using Algebra::PolynomialDegree;
using Algebra::PolynomialInterface;


namespace simple_add {
namespace ACSP_FOR_ADD{

namespace { // anonymous namespace for polys and common vars
    using Algebra::mapIntegerToFieldElement;

class polyAdd_class : public PolynomialInterface {
    public:
        polyAdd_class() {};

        Algebra::FieldElement eval(const std::vector<FieldElement>& x)const{
            return Add::evalp::ep(x);
        }

        bool isEffectiveInput(const size_t varId)const{
            switch(varId)
            {
                // case Add::NUMREGS + Add::reg::ST3: return false;
                // case Add::NUMREGS + Add::reg::invRC: return false;
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

            const ttdeg A = ttdeg(inputDegrees[Add::reg::A]);          
            const ttdeg A_next = ttdeg(inputDegrees[Add::reg::A + Add::NUMREGS]);
            const ttdeg B = ttdeg(inputDegrees[Add::reg::B]);
            const ttdeg B_next = ttdeg(inputDegrees[Add::reg::B + Add::NUMREGS]);
            const ttdeg C = ttdeg(inputDegrees[Add::reg::C]);
            const ttdeg C_next = ttdeg(inputDegrees[Add::reg::C + Add::NUMREGS]);

            const ttdeg resTmp = (A + B + C) + (A_next + A) + (B_next + B) + (C_next + C);

            return PolynomialDegree(resTmp.deg_);
        }

        std::unique_ptr<PolynomialInterface> clone()const{
            return std::unique_ptr<PolynomialInterface>(new polyAdd_class);
        }

        size_t numVars()const{
            return Add::NUMREGS;
        }
};
} 

    Add_CS::Add_CS() {
        polys_.push_back(polyPtr_t(new polyAdd_class()));
    }

    Add_CS* Add_CS::clone() const{
        return new Add_CS();
    }

    using std::vector;
    vector<FieldElement> Add_CS::eval(const vector<FieldElement>& assignment) const{
        vector<FieldElement> res;
        for(const auto& p: polys_){
            res.push_back(p->eval(assignment));
        }
        return res;
    }

} // ACSP_FOR_ADD namespace
} // simple_add namespace