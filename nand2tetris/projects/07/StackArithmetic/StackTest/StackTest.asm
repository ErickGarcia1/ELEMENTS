@256
D=A
@0
M=D
// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D-M
@true.0
D;JEQ
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@END.0
0;JMP
(true.0)
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
(END.0)
// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant 16
@16
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D-M
@true.1
D;JEQ
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@END.1
0;JMP
(true.1)
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
(END.1)
// push constant 16
@16
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D-M
@true.2
D;JEQ
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@END.2
0;JMP
(true.2)
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
(END.2)
// push constant 892
@892
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
// lt
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D-M
@true.3
D;JLT
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@END.3
0;JMP
(true.3)
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
(END.3)
// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant 892
@892
D=A
@SP
A=M
M=D
@SP
M=M+1
// lt
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D-M
@true.4
D;JLT
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@END.4
0;JMP
(true.4)
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
(END.4)
// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
// lt
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D-M
@true.5
D;JLT
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@END.5
0;JMP
(true.5)
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
(END.5)
// push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
// gt
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D-M
@true.6
D;JGT
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@END.6
0;JMP
(true.6)
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
(END.6)
// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
// gt
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D-M
@true.7
D;JGT
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@END.7
0;JMP
(true.7)
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
(END.7)
// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
// gt
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D-M
@true.8
D;JGT
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@END.8
0;JMP
(true.8)
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
(END.8)
// push constant 57
@57
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant 31
@31
D=A
@SP
A=M
M=D
@SP
M=M+1
// push constant 53
@53
D=A
@SP
A=M
M=D
@SP
M=M+1
// add
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D+M;
@SP
A=M
M=D
@SP
M=M+1
// push constant 112
@112
D=A
@SP
A=M
M=D
@SP
M=M+1
// sub
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D-M;
@SP
A=M
M=D
@SP
M=M+1
// neg
@SP
M=M-1
A=M
D=M
D=!D
D=D+1
@SP
A=M
M=D
@SP
M=M+1
// and
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D&M
@SP
A=M
M=D
@SP
M=M+1
// push constant 82
@82
D=A
@SP
A=M
M=D
@SP
M=M+1
// or
@SP
M=M-1
A=M
D=M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
D=D|M
@SP
A=M
M=D
@SP
M=M+1
// not
@SP
M=M-1
A=M
D=M
D=!D
@SP
A=M
M=D
@SP
M=M+1
// **Sample command**
