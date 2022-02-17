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
    D = A   // Set the counter value 
    @counter
    M = D

(LOOP)  // Main loop
    @KBD
    D = M   // Put the data in the keyboard RAM location in register D

    @SET_WHITE
    D;JEQ   // If there is no keypress, jump to where the screen is set to white

// Find the screen, and set it to black
(SET_BLACK) 
    @SCREEN     // Set the D register to the screen register
    D = A
    // Add the offset into the address of the screen and increment the offset
    @offset
    D = D + M
    M = M + 1
    // Set the new address to SCREEN + offset, and set the pixels to black
    A = D
    M = -1
    // Compare the offset with the counter value. If they are equal, exit the loop
    @offset
    D = M
    @counter
    D = D - M
    @END_SET    // If all pixels are set to black, jump to end of the set function
    D - M;JEQ

    @SET_BLACK
    0;JMP

(SET_WHITE)
    @SCREEN     // Set the D register to the screen register
    D = A
    // Add the offset into the address of the screen and increment the offset
    @offset
    D = D + M
    M = M + 1
    // Set the new address to SCREEN + offset, and set the pixels to white
    A = D
    M = 0
    // Compare the offset with the counter value. If they are equal, exit the loop
    @offset
    D = M
    @counter
    D = D - M
    @END_SET    // If all pixels are set to black, jump to end of the set function
    D - M;JEQ

    @SET_WHITE
    0;JMP

(END_SET)
    @offset
    M = 0   // Reset the offset to 0
    @LOOP
    0;JMP   // Loop back
