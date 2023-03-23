#ifndef __ACSP_FOR_MIPS_INSTANCE_HPP__
#define __ACSP_FOR_MIPS_INSTANCE_HPP__

#include "mips.hpp"
// #include "languages/Bair/ConstraintsSys.hpp"
// #include "languages/Bair"

using namespace libstark;
namespace simple_mips{
namespace ACSP_FOR_MIPS{

    class Mips_CS : public libstark::ConstraintSys{
    public:
        Mips_CS(); 
    
    // the amount of inputs each polynomial expects
    size_t numVars() const{ return 2 * mips::NUMREGS; }
    size_t numMappings() const{ return polys_.size(); }

    Mips_CS* clone() const;

    // the constraint polynomials
    const polySet_t& constraints() const{ return polys_; }
    std::vector<FieldElement> eval(const std::vector<FieldElement>& assignment) const;

    private:
        polySet_t polys_;

    };

} // namespace ACSP_FOR_MIPS
} // namespace simple_mips



#endif // __ACSP_FOR_MIPS_INSTANCE_HPP__
