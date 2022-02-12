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

// @1
// D = A   //Load 1 into D
// @16
// M = D   //Load 1 into RAM[16]
// @2
// D = M   //Load RAM[2] into D
// @17
// M = D   //Load contents of RAM[2] into RAM[17]
// @16
// D = M   //Load bitmask into D
// M = M + 1

@1
D = M
@2
M = D
@0
D = M
@2
M = D + M

(END)
    @END
    0;JMP
