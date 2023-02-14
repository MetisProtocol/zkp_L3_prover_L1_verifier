./stark-dpm examples-dpm/database.txt examples-dpm/fp_no_match.txt

-------------------------------------------------------------
| BAIR Specifications                                       |
-------------------------------------------------------------
| field size                                     = 2^64     |
| number of variables per state (w)              = 81       |
| number of regular constraints (s)              = 1        |
| number of permutation constraints              = 0        |
| number of cycles (c)                           = (2^12)-1 |
| degree of constraint system (d)                = 8        |
| degree of permutation constraint system        = -1       |
| number of boundary constraints (B)             = 4        |
| number of variables used by constraint systems = 81       |
| number of variables routed                     = 0        |
| number of variables not routed                 = 81       |
-------------------------------------------------------------
--------------------------------------------------
| ACSP Specifications                            |
--------------------------------------------------
| field size                             = 2^64  |
| number of algebraic-registers (|\Tau|) = 81    |
| number of neighbors (|N|)              = 239   |
| vanishing space size                   = 2^13  |
| composition degree bound               = 73720 |
--------------------------------------------------
-------------------------------------------------------------
| APR Specifications                                        |
-------------------------------------------------------------
| field size                                       = 2^64   |
| number of algebraic-registers (|\Tau|)           = 81     |
| number of neighbors (|N|)                        = 239    |
| witness (f) evaluation space size (|L|)          = 2^21   |
| constraint (g) evaluation space size (|L_{cmp}|) = 2^19   |
| witness (f) maximal rate (\rho_{max})            = 2^{-8} |
| constraint (g) rate (\rho_{cmp})                 = 2^{-3} |
| zero knowledge parameter (k)                     = 1      |
| rate parameter (R)                               = 3      |
| constraints degree log (d)                       = 16     |
-------------------------------------------------------------
Constructing APR (ACSP) witness:..(0.524379 seconds)
-----------------------------------------
| FRI for witness (f) specifications #1 |
-----------------------------------------
| field size (|F|)      = 2^64          |
| RS code dimension     = 2^13          |
| RS block-length       = 2^21          |
| RS code rate          = 2^-{8}        |
| Soundness error       = 2^-{61}       |
| dim L_0 (eta)         = 2             |
| recursion depth       = 6             |
| COMMIT repetitions    = 2             |
| number of tests (ell) = 8             |
-----------------------------------------
---------------------------------------------
| FRI for constraints (g) specifications #1 |
---------------------------------------------
| field size (|F|)      = 2^64              |
| RS code dimension     = 2^16              |
| RS block-length       = 2^19              |
| RS code rate          = 2^-{3}            |
| Soundness error       = 2^-{61}           |
| dim L_0 (eta)         = 2                 |
| recursion depth       = 8                 |
| COMMIT repetitions    = 2                 |
| number of tests (ell) = 21                |
---------------------------------------------
communication iteration #1:.......(15.1419 seconds)
communication iteration #2:.....(7.0487 seconds)
communication iteration #3:..(0.289465 seconds)
communication iteration #4:.(0.102203 seconds)
communication iteration #5:.(0.045203 seconds)
communication iteration #6:.(0.029624 seconds)
communication iteration #7:.(0.024087 seconds)
communication iteration #8:.(0.013464 seconds)
communication iteration #9:.(0.009925 seconds)
communication iteration #10:.(0.021169 seconds)
communication iteration #11:.(0.03392 seconds)
communication iteration #12:.(0.143821 seconds)
Verifier decision: ACCEPT
------------------------------------------------------
| Protocol execution measurements                    |
------------------------------------------------------
| Prover time                    = 22.728386 Seconds |
| Verifier time                  = 0.132197 Seconds  |
| Total proof oracles size       = 4.093750 GBytes   |
| Total communication complexity = 270.390625 KBytes |
| Query complexity               = 191.843750 KBytes |
------------------------------------------------------
