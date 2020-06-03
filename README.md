# smallvm
A very minimal virtual machine written in rust.
The virtual machine has:
-   8 Registers
-   A stack
-   A heap
-   Data types include: u8, i8, u16, i16, u32, i32, u64, i64, f32, f64

The instruction set includes: 
-    NOP(),                          //do nothing
-    MOV(Register, Variable),        //mov variable to reg
-    MOVR(Register, Register),       //mov reg contents to another reg
-    JMP(Register),                  //jump to location
-    JE(Register),                   //Jump if zero to location
-    JNE(Register),                  //Jump if not zero to location
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
-    VPUSH(Variable),                //Push variable on to the stack
-    VPUSHR(Register),               //Push register contents on the stack
-    VPOP(Register),                 //pops variable from stack to register
-    CALL(Register),                 //call functon at address in register
-    RET(),                          //return from routine
-    HALT(),                         //halt the program