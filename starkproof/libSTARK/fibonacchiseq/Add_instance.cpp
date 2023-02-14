#include "Add_instance.hpp"


namespace simple_add{
namespace ACSP_FOR_ADD{

namespace{

using std::vector;

class empty_CS : public libstark::ConstraintSys{
    public:

    /// the amount of inputs each polynomial expects
	size_t numVars() const{ return 2 * Add::NUMREGS; }
    size_t numMappings() const{return polys.size();}
    empty_CS* clone() const{return new empty_CS();}

    /// the constraint polynomials
	virtual const polySet_t& constraints() const{return polys;}
    vector<FieldElement> eval(const vector<FieldElement>& assignment) const{
        vector<FieldElement> res;
        for(const auto& p : polys){
            res.push_back(p->eval(assignment));
        }

        return res;
    }

private:
	polySet_t polys;
};
}

libstark::BairInstance buildBairInstance(const unsigned int a, const unsigned int b) {
    using Algebra::FieldElement;
    using Algebra::zero;
    using Algebra::mapIntegerToFieldElement;
    using libstark::BairInstance;

    const size_t vectorsLen = Add::NUMREGS;
    // const short domainSizeIndicator = Add::getDim(params.length);
    
    BairInstance::constraintsPtr_t constraints_assignment(new Add_CS());
    BairInstance::constraintsPtr_t constraints_permutation(new empty_CS());

    BairInstance::boundaryConstraints_t boundary;
    boundary[BairInstance::point_t(0, Add::reg::A)] = mapIntegerToFieldElement(0, 64, a);
    boundary[BairInstance::point_t(0, Add::reg::B)] = mapIntegerToFieldElement(0, 64, b);
    
    const short domainSizeIndicator = (short)Infrastructure::Log2(Add::lastStep + 1);
        
    boundary[BairInstance::point_t(Add::lastStep, Add::reg::C)] 
                = mapIntegerToFieldElement(0, 64, 52) + mapIntegerToFieldElement(0, 64, 9);

    return BairInstance(vectorsLen, 
                        domainSizeIndicator, 
                        std::move(constraints_assignment), 
                        std::move(constraints_permutation), 
                        boundary,
                        std::vector<Algebra::FieldElement>(Add::NUMREGS, zero())
                        );
}

} // namespace ACSP_FOR_ADD
} // namespace simple_add

