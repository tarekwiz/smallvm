# smallvm
A very minimal virtual machine written in rust.
### The virtual machine has:
-   8 Registers
-   A stack
-   A heap
-   Data types include: u8, i8, u16, i16, u32, i32, u64, i64, f32, f64

### The instruction set includes: 
-    NOP(),                          //do nothing
-    MOV(Register, Variable),        //mov variable to reg
-    MOVR(Register, Register),       //mov reg contents to another reg
-    JMP(Register),                  //jump to location
-    JE(Register),                   //Jump if equal to location
-    JNE(Register),                  //Jump if not equal to location
-    JG(Register),                   //Jump if greater than
-    JL(Register),                   //Jump if less than
-    CMP(Register, Register),        //Compares two registers
-    PRINTR(Register),               //print contents of register
-    PRINTV(Address),                //print contents of variable at address
-    VSTORE(Address, Variable),      //store variable into VMHeap at specific address from stack
-    VLOAD(Address),                 //load variable from VMHeap and pushes value to stack
-    VSTORER(Address, Register),     //store variable into VMHeap from register contents
-    VLOADR(Register, Address),      //loads a variable from VMHeap to register
-    ADD(Register, Register),        //Add 2 registers and pushes result on stack
-    SUB(Register, Register),        //Subtract 2 registers and pushes result on stack
-    MUL(Register, Register),        //Multiple 2 registers and pushes result on stack
-    DIV(Register, Register),        //Divide 2 registers and pushes result on stack
-    AND(Register, Register),        //Bitwise AND on 2 registers. pushes result on stack
-    OR(Register, Register),         //Bitwise OR on 2 registers. pushes result on stack
-    XOR(Register, Register),        //Bitwise XOR on 2 registers. pushes result on stack
-    SHR(Register, Variable),        //Shifts register to the right by (Variable)
-    SHL(Register, Variable),        //Shifts register to the left by (Variable)
-    VPUSH(Variable),                //Push variable on to the stack
-    VPUSHR(Register),               //Push register contents on the stack
-    VPOP(Register),                 //pops variable from stack to register
-    CALL(Register),                 //call functon at address in register
-    RET(),                          //return from routine
-    HALT(),                         //bye bye

#### Example Program:
`1 0 0 10`     `MOV(R0, 10)`  
`1 1 0 8`      `MOV(R1, 8)`  
`1 2 0 22`     `MOV(R2, 23) Location to jump to if R0 is greater than R1`  
`1 3 0 25`     `MOV(R3, 25) Location to jump to otherwise`  
`6 0 1`        `CMP(R0, R1)`  
`23 2`         `JG(R2) Jump if R0 is greater than R1`  
`3 3`          `JMP(R3)`  
`7 0`          `PRINTR(R0)`  
`22`           `HALT()`  
`7 1`          `PRINTR(R1)`  
`22`           `HALT()`  
