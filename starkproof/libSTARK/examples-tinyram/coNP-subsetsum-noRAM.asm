MOV r0 r0 1
CMPE r0 r0 8
CJMP r0 r0 20
MOV r1 r0 0
MOV r2 r0 r0
MOV r3 r0 0
AND r4 r2 1
CMPE r0 r4 0
CJMP r0 r0 11
RESERVED_OPCODE_24 r4 r0 r3
ADD r1 r1 r4
SHR r2 r2 1
CMPE r0 r2 0
CJMP r0 r0 16
ADD r3 r3 1
JMP r0 r0 6
CMPE r0 r1 0
CJMP r0 r0 21
ADD r0 r0 1
JMP r0 r0 1
MOV r0 r0 0
ANSWER r0 r0 r0
