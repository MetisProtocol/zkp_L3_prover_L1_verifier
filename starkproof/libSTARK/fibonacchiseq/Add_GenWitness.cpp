#include "Add.hpp"

#define CYCLES 62 // TODO: Fix number

using namespace Algebra;


namespace Add {      

    #define DBGGET(a,b,c) (a)[(b)][(c)]  

    void genWitnessAddWithPadding(witnessType arr, const unsigned int a, const unsigned int b) {                               
        
        DBGGET(arr, 0, reg::A) = mapIntegerToFieldElement(0, 64, a);
        DBGGET(arr, 0, reg::B) = mapIntegerToFieldElement(0, 64, b);
        DBGGET(arr, 0, reg::C) = arr[0][reg::A] + arr[0][reg::B];

        for (size_t i = 1;i < Add::lastStep + 1; i++) {                                    
             DBGGET(arr, i, reg::A) = arr[i - 1][reg::B];
             DBGGET(arr, i, reg::B) = arr[i - 1][reg::C];
             DBGGET(arr, i, reg::C) = arr[i][reg::A] + arr[i][reg::B];
        }                
    }

    short getDim(long long len)
    {
        return ceil(Infrastructure::Log2((long long)CYCLES * (len + 1) - 2));
        // return ceil(Infrastructure::Log2(59LL * len));
    }
} // namespace