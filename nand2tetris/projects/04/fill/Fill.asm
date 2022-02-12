// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.
    @offset
    M = 0   // Set the offset to 0
    @8192
    D = A
    @counter
    M = D
    @set_color
    M = 0


(LOOP)  // Main loop
    @KBD
    D = M   // Put the data in the keyboard RAM location in register D

    @SET_WHITE
    D;JEQ   // If there is no keypress, jump to where the screen is set to white

// Find the screen, and change the value 
(SET_BLACK) 
    @SCREEN
    D = A
    @offset
    D = D + M
    M = M + 1
    A = D
    M = -1
    @offset
    D = M
    @counter
    D = D - M
    @END_SET
    D;JEQ

    @SET_BLACK    // When all pixels are set to black, jump to end of the set function
    0;JMP

(SET_WHITE)
    @SCREEN
    D = A
    @offset
    D = D + M
    M = M + 1
    A = D
    M = 0
    @offset
    D = M
    @counter
    D = D - M
    @END_SET
    D;JEQ

    @SET_WHITE
    0;JMP

(END_SET)
    @offset
    M = 0   // Set the offset to 0
    @8192
    D = A
    @counter
    M = D
    @LOOP
    0;JMP
