# smallvm
A very minimal virtual machine written in rust.
### The virtual machine has:
-   8 Registers
-   A stack
-   A heap
-   Data types include: u8, i8, u16, i16, u32, i32, u64, i64, f32, f64

## Instruction Set

### Root assembly instructions

| Instruction | Left Input | Right Input | Operation |
| ----------- | ---------- | ----------- | --------- |
| NOP         |            |             | No Operation |
| MOV         | Register   | Variable    | Move variable to register |
| MOVR        | Register   | Register    | Move register contents to another register |
| JMP         | Register   |             | Jump to memory location |
| JE          | Register   |             | Jump if equal |
| JNE         | Register   |             | Jump not equal |
| JG          | Register   |             | Jump if greater |
| JL          | Register   |             | Jump if less |
| CMP         | Register   | Register    | Compare two registers |
| ADD         | Register   | Register    | Add 2 registers, push result to stack |
| SUB         | Register   | Register    | Subtract 2 registers, push result to stack |
| MUL         | Register   | Register    | Multiply 2 registers, push result to stack |
| DIV         | Register   | Register    | Divide 2 registers, push result to stack |
| AND         | Register   | Register    | Bitwise AND on 2 registers, push result to stack |
| OR          | Register   | Register    | Bitwise OR on 2 registers, push result to stack |
| XOR         | Register   | Register    | Bitwise Exclusive OR on 2 registers, push result to stack |
| SHR         | Register   | Variable    | Shift register to the right by Variable |
| SHL         | Register   | Variable    | Shift register to the left by Variable |
| VPUSH       | Variable   |             | Push variable contents on to stack |
| VPUSHR      | Register   |             | Push register contents on to stack |
| VPOP        | Variable   |             | Pops variable from stack to register |
| CALL        | Register   |             | Calls function at address in the register |
| RET         |            |             | Return from routine |
| HALT        |            |             | Halt CPU/Exit |
| VSTORE      | Address    | Variable    | Store var into VMHeap at specific address from stack |
| VLOAD       | Address    |             | Load var from VMHeap and push value to stack |
| VSTORER     | Address    | Register    | Store var in VMHeap from register contents |
| VLOADR      | Register   | Address     | Loads a variable from VMHeap to register |

### Debugging assembly instructions

| Instruction | Left Input | Right Input | Operation |
| ----------- | ---------- | ----------- | --------- |
| PRINTR      | Register   |             | Print contents of register |
| PRINTV      | Address    |             | Print contents of variable at address |

## Example Program:

```
1 0 0 10    MOV(R0, 10)  
1 1 0 8     MOV(R1, 8)  
1 2 0 22    MOV(R2, 23) Location to jump to if R0 is greater than R1  
1 3 0 25    MOV(R3, 25) Location to jump to otherwise  
6 0 1       CMP(R0, R1)  
23 2        JG(R2) Jump if R0 is greater than R1  
3 3         JMP(R3)  
7 0         PRINTR(R0)  
22          HALT()  
7 1         PRINTR(R1)  
22          HALT()  
```

