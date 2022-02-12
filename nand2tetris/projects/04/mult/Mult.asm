// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// Put your code here.
@2
M = 0   // Reset value in RAM[2]
@1
D = M   // Load value from RAM[1] into D
@3
M = D   // Put value from RAM[1] into RAM[3] to serve as a loop counter
@1
D = M
@SKIP_LOOP
D;JEQ

(MULT_LOOP)
    @0
    D = M       // Load value from RAM[0]
    @2
    M = M + D   // Add value from RAM[0] into RAM[2]
    @3
    M = M - 1   // Decrement counter
    D = M
    // If counter has not reached 0, loop
    @MULT_LOOP
    D;JNE
(SKIP_LOOP)
(END)
    @END
    0;JMP
