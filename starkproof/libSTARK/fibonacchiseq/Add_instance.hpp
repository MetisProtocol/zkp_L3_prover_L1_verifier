#ifndef __ACSP_FOR_ADD_INSTANCE_HPP__
#define __ACSP_FOR_ADD_INSTANCE_HPP__

#include "Add.hpp"
#include "languages/Bair/ConstraintsSys.hpp"

namespace simple_add{
namespace ACSP_FOR_ADD{

    class Add_CS : public libstark::ConstraintSys{
    public:
        Add_CS();
    
    // the amount of inputs each polynomial expects
    size_t numVars() const{ return 2 * Add::NUMREGS; }
    size_t numMappings() const{ return polys_.size(); }

    Add_CS* clone() const;

    // the constraint polynomials
    const polySet_t& constraints() const{ return polys_; }
    std::vector<FieldElement> eval(const std::vector<FieldElement>& assignment) const;

    private:
        polySet_t polys_;

    };

} // namespace ACSP_FOR_ADD
} // namespace simple_add



#endif // __ACSP_FOR_ADD_INSTANCE_HPP__