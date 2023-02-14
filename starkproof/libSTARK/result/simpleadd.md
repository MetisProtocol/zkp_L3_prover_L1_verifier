./simple-add 1 2

------------------------------------------------------------
| BAIR Specifications                                      |
------------------------------------------------------------
| field size                                     = 2^64    |
| number of variables per state (w)              = 1       |
| number of regular constraints (s)              = 1       |
| number of permutation constraints              = 0       |
| number of cycles (c)                           = (2^7)-1 |
| degree of constraint system (d)                = 1       |
| degree of permutation constraint system        = -1      |
| number of boundary constraints (B)             = 1       |
| number of variables used by constraint systems = 1       |
| number of variables routed                     = 0       |
| number of variables not routed                 = 1       |
------------------------------------------------------------
-------------------------------------------------
| ACSP Specifications                           |
-------------------------------------------------
| field size                             = 2^64 |
| number of algebraic-registers (|\Tau|) = 1    |
| number of neighbors (|N|)              = 3    |
| vanishing space size                   = 2^8  |
| composition degree bound               = 511  |
-------------------------------------------------
-------------------------------------------------------------
| APR Specifications                                        |
-------------------------------------------------------------
| field size                                       = 2^64   |
| number of algebraic-registers (|\Tau|)           = 1      |
| number of neighbors (|N|)                        = 3      |
| witness (f) evaluation space size (|L|)          = 2^13   |
| constraint (g) evaluation space size (|L_{cmp}|) = 2^11   |
| witness (f) maximal rate (\rho_{max})            = 2^{-5} |
| constraint (g) rate (\rho_{cmp})                 = 2^{-3} |
| zero knowledge parameter (k)                     = 1      |
| rate parameter (R)                               = 3      |
| constraints degree log (d)                       = 8      |
-------------------------------------------------------------
Constructing APR (ACSP) witness:.(0.021075 seconds)
-----------------------------------------
| FRI for witness (f) specifications #1 |
-----------------------------------------
| field size (|F|)      = 2^64          |
| RS code dimension     = 2^8           |
| RS block-length       = 2^13          |
| RS code rate          = 2^-{5}        |
| Soundness error       = 2^-{61}       |
| dim L_0 (eta)         = 2             |
| recursion depth       = 4             |
| COMMIT repetitions    = 2             |
| number of tests (ell) = 13            |
-----------------------------------------
---------------------------------------------
| FRI for constraints (g) specifications #1 |
---------------------------------------------
| field size (|F|)      = 2^64              |
| RS code dimension     = 2^8               |
| RS block-length       = 2^11              |
| RS code rate          = 2^-{3}            |
| Soundness error       = 2^-{61}           |
| dim L_0 (eta)         = 2                 |
| recursion depth       = 4                 |
| COMMIT repetitions    = 2                 |
| number of tests (ell) = 21                |
---------------------------------------------
communication iteration #1:.(0.025372 seconds)
communication iteration #2:.(0.037487 seconds)
communication iteration #3:.(0.035184 seconds)
communication iteration #4:.(0.031507 seconds)
communication iteration #5:.(0.032925 seconds)
communication iteration #6:.(0.025916 seconds)
communication iteration #7:.(0.019407 seconds)
communication iteration #8:.(0.022503 seconds)
Verifier decision: ACCEPT
------------------------------------------------------
| Protocol execution measurements                    |
------------------------------------------------------
| Prover time                    = 0.052157 Seconds  |
| Verifier time                  = 0.009585 Seconds  |
| Total proof oracles size       = 640.035156 KBytes |
| Total communication complexity = 30.500000 KBytes  |
| Query complexity               = 6.953125 KBytes   |
------------------------------------------------------
