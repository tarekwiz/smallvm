use std::mem;
use std::convert::TryInto;
use derive_more::*;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Add, Sub)]
enum Immediate {
    None(),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64)
}

type Register = usize;
type Address = usize;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    NOP(),                          //do nothing
    MOV(Register, Immediate),       //mov immediate to reg
    MOVR(Register, Register),       //mov reg contents to another reg
    JMP(Register),                  //jump to location
    JE(Register),                   //Jump if equal to location
    JNE(Register),                  //Jump if not equal to location
    JG(Register),                   //Jump if greater than
    JL(Register),                   //Jump if less than
    CMP(Register, Register),        //Compares two registers
    PRINTR(Register),               //print contents of register
    PRINTV(Address),                //print contents of immediate at address
    VSTORE(Address, Immediate),     //store immediate into VMHeap at specific address from stack
    VLOAD(Address),                 //load immediate from VMHeap and pushes value to stack
    VSTORER(Address, Register),     //store immediate into VMHeap from register contents
    VLOADR(Register, Address),      //loads a immediate from VMHeap to register
    ADD(Register, Register),        //Add 2 registers and pushes result on stack
    SUB(Register, Register),        //Subtract 2 registers and pushes result on stack
    MUL(Register, Register),        //Multiple 2 registers and pushes result on stack
    DIV(Register, Register),        //Divide 2 registers and pushes result on stack
    AND(Register, Register),        //Bitwise AND on 2 registers. pushes result on stack
    OR(Register, Register),         //Bitwise OR on 2 registers. pushes result on stack
    XOR(Register, Register),        //Bitwise XOR on 2 registers. pushes result on stack
    SHR(Register, Immediate),       //Shifts register to the right by (immediate)
    SHL(Register, Immediate),       //Shifts register to the left by (immediate)
    VPUSH(Immediate),               //Push immediate on to the stack
    VPUSHR(Register),               //Push register contents on the stack
    VPOP(Register),                 //pops immediate from stack to register
    CALL(Register),                 //call functon at address in register
    RET(),                          //return from routine
    HALT(),                         //bye bye
}

struct VirtualMachine {
    ip : Address,
    flag_eq : bool,
    flag_gt: bool,
    reg : [Immediate; 8],
    code : Vec<u8>,
    stack : Vec<Immediate>,
    data : Vec<Immediate>,
    is_executing : bool,
}

impl VirtualMachine {
    fn new(c : Vec<u8>, heap_capacity: usize) -> Self {
        VirtualMachine { ip: 0, flag_eq: false, flag_gt: false, reg: [Immediate::U8(0); 8], code: c, stack: Vec::new(), data: vec![Immediate::U8(0); heap_capacity], is_executing: false } 
    }
    fn decode_immediate(&mut self) -> Immediate {
        self.ip += 1;
        match self.code[self.ip] {
            0 => {
                self.ip += 1;
                Immediate::U8(self.code[self.ip] as u8)
            },
            1 => {
                self.ip += 1;
                Immediate::I8(self.code[self.ip] as i8)
            },
            2 => {
                self.ip += 1;
                let size = mem::size_of::<u16>();
                let value = u16::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::U16(value)
            },
            3 => {
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = i16::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::I16(value)
            },
            4 => {
                self.ip += 1;
                let size = mem::size_of::<u32>();
                let value = u32::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::U32(value)
            },
            5 => {
                self.ip += 1;
                let size = mem::size_of::<i32>();
                let value = i32::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::I32(value)
            },
            6 => {
                self.ip += 1;
                let size = mem::size_of::<u64>();
                let value = u64::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::U64(value)
            },
            7 => {
                self.ip += 1;
                let size = mem::size_of::<i64>();
                let value = i64::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::I64(value)
            },
            8 => {
                self.ip += 1;
                let size = mem::size_of::<f32>();
                let value = f32::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::F32(value)
            },
            9 => {
                self.ip += 1;
                let size = mem::size_of::<f64>();
                let value = f64::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::F64(value)
            },
            _ => Immediate::None()
        }
    }

    fn decode(&mut self) -> Instruction {
        match self.code[self.ip] {
            0 => Instruction::NOP(),
            1 => {
                self.ip += 1;
                let register = self.code[self.ip] as Register;
                let var = self.decode_immediate();
                Instruction::MOV(register, var)
            },
            2 => {
                self.ip += 1;
                let reg1 = self.code[self.ip] as Register;
                self.ip += 1;
                let reg2 = self.code[self.ip] as Register;
                Instruction::MOVR(reg1, reg2)
            },
            3 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::JMP(reg)
            },
            4 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::JE(reg)
            },
            5 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::JNE(reg)
            },
            6 => {
                self.ip += 1;
                let reg1 = self.code[self.ip] as Register;
                self.ip += 1;
                let reg2 = self.code[self.ip] as Register;
                Instruction::CMP(reg1, reg2)
            },
            7 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::PRINTR(reg)
            },
            8 => {
                self.ip += 1;
                let addr = self.code[self.ip] as Address;
                Instruction::PRINTV(addr)
            },
            9 => {
                self.ip += 1;
                let addr = self.code[self.ip] as Address;
                let var = self.decode_immediate();
                Instruction::VSTORE(addr, var)
            },
            10 => {
                self.ip += 1;
                let addr = self.code[self.ip] as Address;
                Instruction::VLOAD(addr)
            },
            11 => {
                self.ip += 1;
                let reg1 = self.code[self.ip] as Register;
                self.ip += 1;
                let reg2 = self.code[self.ip] as Register;
                Instruction::ADD(reg1, reg2)
            },
            12 => {
                self.ip += 1;
                let reg1 = self.code[self.ip] as Register;
                self.ip += 1;
                let reg2 = self.code[self.ip] as Register;
                Instruction::SUB(reg1, reg2)
            },
            13 => {
                self.ip += 1;
                let reg1 = self.code[self.ip] as Register;
                self.ip += 1;
                let reg2 = self.code[self.ip] as Register;
                Instruction::MUL(reg1, reg2)
            },
            14 => {
                self.ip += 1;
                let reg1 = self.code[self.ip] as Register;
                self.ip += 1;
                let reg2 = self.code[self.ip] as Register;
                Instruction::DIV(reg1, reg2)
            },
            15 => {
                self.ip += 1;
                let addr = self.code[self.ip] as Address;
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::VSTORER(addr, reg)
            },
            16 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                self.ip += 1;
                let addr = self.code[self.ip] as Address;
                Instruction::VLOADR(reg, addr)
            },
            17 => {
                let var = self.decode_immediate();
                Instruction::VPUSH(var)
            },
            18 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::VPUSHR(reg)
            },
            19 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::VPOP(reg)
            },
            20 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::CALL(reg)
            },
            21 => Instruction::RET(),
            22 => Instruction::HALT(),
            23 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::JG(reg)
            },
            24 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                Instruction::JL(reg)
            },
            25 => {
                self.ip += 1;
                let reg1 = self.code[self.ip] as Register;
                self.ip += 1;
                let reg2 = self.code[self.ip] as Register;
                Instruction::AND(reg1, reg2)
            },
            26 => {
                self.ip += 1;
                let reg1 = self.code[self.ip] as Register;
                self.ip += 1;
                let reg2 = self.code[self.ip] as Register;
                Instruction::OR(reg1, reg2)
            },
            27 => {
                self.ip += 1;
                let reg1 = self.code[self.ip] as Register;
                self.ip += 1;
                let reg2 = self.code[self.ip] as Register;
                Instruction::XOR(reg1, reg2)
            },
            28 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                let var = self.decode_immediate();
                Instruction::SHR(reg, var)
            },
            29 => {
                self.ip += 1;
                let reg = self.code[self.ip] as Register;
                let var = self.decode_immediate();
                Instruction::SHL(reg, var)
            },
            _ => Instruction::NOP(),
        }
    }
    fn execute(&mut self, instr: Instruction) -> bool
    {
        println!("Executing: {:?} \t  current ip: {:?}", instr, self.ip + 1);
        match instr {
            Instruction::NOP() => true,
            Instruction::MOV(reg, var) => {
                self.reg[reg] = var;
                true
            },
            Instruction::MOVR(reg1, reg2) => {
                self.reg[reg1] = self.reg[reg2];
                true
            },
            Instruction::JMP(reg) => {
                match self.reg[reg] {
                    Immediate::U8(v) => {
                        self.ip = v as Address;
                    true
                    }
                    Immediate::U16(v) => {
                        self.ip = v as Address;
                        true
                    }
                    _ => false
                }
            },
            Instruction::JE(reg) => {
                if !self.flag_eq {
                    return true;
                }
                self.execute(Instruction::JMP(reg))
            },
            Instruction::JNE(reg) => {
                if self.flag_eq {
                    return true;
                }
                self.execute(Instruction::JMP(reg))
            },
            Instruction::JG(reg) => {
                if !self.flag_gt {
                    return true;
                }
                self.execute(Instruction::JMP(reg))
            },
            Instruction::JL(reg) => {
                if self.flag_gt {
                    return true;
                }
                self.execute(Instruction::JMP(reg))
            },
            Instruction::CMP(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                self.flag_eq = v1 == v2;
                self.flag_gt = v1 > v2;
                true
            },
            Instruction::PRINTR(reg) => {
                let val = &self.reg[reg];
                println!("Printing: {:?}", val);
                true
            },
            Instruction::PRINTV(addr) => {
                let val = &self.data[addr];
                println!("Printing: {:?}", val);
                true
            },
            Instruction::VSTORE(addr, var) => {
                self.data[addr] = var;
                true
            },
            Instruction::VLOAD(addr) => {
                self.stack.push(self.data[addr]);
                true
            },
            Instruction::VSTORER(addr, reg) => {
                self.data[addr] = self.reg[reg];
                true
            },
            Instruction::VLOADR(reg, addr) => {
                self.reg[reg] = self.data[addr];
                true
            },
            Instruction::ADD(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                if let Ok(r) = v1 + v2 {
                    self.stack.push(r);
                    true
                } else {
                    false
                }
            },
            Instruction::SUB(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                if let Ok(r) = v1 - v2 {
                    self.stack.push(r);
                    true
                } else {
                    false
                }
            },
            Instruction::MUL(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u*v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u*v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u*v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u*v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u*v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u*v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u*v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u*v));
                    },
                    (Immediate::F32(v), Immediate::F32(u)) => {
                        self.stack.push(Immediate::F32(u*v));
                    },
                    (Immediate::F64(v), Immediate::F64(u)) => {
                        self.stack.push(Immediate::F64(u*v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::DIV(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u/v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u/v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u/v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u/v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u/v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u/v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u/v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u/v));
                    },
                    (Immediate::F32(v), Immediate::F32(u)) => {
                        self.stack.push(Immediate::F32(u/v));
                    },
                    (Immediate::F64(v), Immediate::F64(u)) => {
                        self.stack.push(Immediate::F64(u/v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::VPUSH(var) => {
                self.stack.push(var);
                true
            },
            Instruction::VPUSHR(reg) => {
                self.stack.push(self.reg[reg]);
                true
            },
            Instruction::VPOP(reg) => {
                match self.stack.pop() {
                    Some(v) => {
                        self.reg[reg] = v;
                        true
                    },
                    _ => false
                }
            },
            Instruction::CALL(reg) => {
                self.stack.push(Immediate::U16(self.ip as u16 + 1));
                self.execute(Instruction::JMP(reg))
            },
            Instruction::OR(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u|v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u|v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u|v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u|v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u|v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u|v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u|v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u|v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::XOR(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u^v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u^v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u^v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u^v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u^v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u^v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u^v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u^v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::SHR(reg1, v2) => {
                let v1 = self.reg[reg1];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u>>v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u>>v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u>>v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u>>v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u>>v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u>>v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u>>v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u>>v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::SHL(reg1, v2) => {
                let v1 = self.reg[reg1];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u<<v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u<<v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u<<v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u<<v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u<<v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u<<v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u<<v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u<<v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::AND(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u&v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u&v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u&v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u&v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u&v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u&v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u&v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u&v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::RET() => {
                match self.stack.pop() {
                    Some(v) => {
                        match v {
                            Immediate::U8(u) => {
                                self.ip = u as usize;
                                true
                            }
                            Immediate::U16(u) => {
                                self.ip = u as usize;
                                true
                            }
                            _ => false
                        }
                    }
                    _ => false
                }
            }
            Instruction::HALT() => {
                self.is_executing = false;
                true
            },
        }
    }

    fn cpu(&mut self) {
        //machine is already running.
        if self.is_executing {
            return;
        }

        self.is_executing = true;

        while self.ip < self.code.len() && self.is_executing
        {
            //decode current instruction
            let instr = self.decode();
            
            //execute instruction
            let result = self.execute(instr);
            //check if instruction execution finished successfully
            if !result {
                panic!("Failed to execute instruction at ip:{:?}", self.ip)
            }

            //go to next instruction
            self.ip += 1;
        }
    }
}
    //Example Program:
    // 1 0 0 10     MOV(R0, 10)
    // 1 1 0 8      MOV(R1, 8)
    // 1 2 0 22     MOV(R2, ?) Location to jump to if R0 is greater than R1
    // 1 3 0 25     MOV(R3, ?) Location to jump to otherwise
    // 6 0 1        CMP(R0, R1)
    // 23 2         JG(R2) Jump if R0 is greater than R1
    // 3 3          JMP(R3)
    // 7 0          PRINTR(R0)
    // 22           HALT()
    // 7 1          PRINTR(R1)
    // 22           HALT()
fn main() {
    let mut vm = VirtualMachine::new(vec![1, 0, 0, 10, 1, 1, 0, 8, 1, 2, 0, 22, 1, 3, 0, 25, 6, 0, 1, 23, 2, 3, 3, 7, 0, 22, 7, 1, 22], 1024);
    vm.cpu();
}
