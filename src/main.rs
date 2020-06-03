use std::mem;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
enum Variable {
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
    MOV(Register, Variable),        //mov variable to reg
    MOVR(Register, Register),       //mov reg contents to another reg
    JMP(Register),                  //jump to location
    JE(Register),                   //Jump if zero to location
    JNE(Register),                  //Jump if not zero to location
    JG(Register),                   //Jump if greater than
    JL(Register),                   //Jump if less than
    CMP(Register, Register),        //Compares two registers
    PRINTR(Register),               //print contents of register
    PRINTV(Address),                //print contents of variable at address
    VSTORE(Address, Variable),      //store variable into VMHeap at specific address from stack
    VLOAD(Address),                 //load variable from VMHeap and pushes value to stack
    VSTORER(Address, Register),     //store variable into VMHeap from register contents
    VLOADR(Register, Address),      //loads a variable from VMHeap to register
    ADD(Register, Register),        //Add 2 registers and pushes result on stack
    SUB(Register, Register),        //Subtract 2 registers and pushes result on stack
    MUL(Register, Register),        //Multiple 2 registers and pushes result on stack
    DIV(Register, Register),        //Divide 2 registers and pushes result on stack
    VPUSH(Variable),                //Push variable on to the stack
    VPUSHR(Register),               //Push register contents on the stack
    VPOP(Register),                 //pops variable from stack to register
    CALL(Register),                 //call functon at address in register
    RET(),                          //return from routine
    HALT(),                         //bye bye
}

struct VirtualMachine {
    ip : Address,
    sp : Address,
    fp : Address,
    flag_eq : bool,
    flag_gt: bool,
    reg : [Variable; 8],
    code : Vec<u8>,
    stack : Vec<Variable>,
    data : Vec<Variable>,
    is_executing : bool,
}

impl VirtualMachine {
    fn new(c : Vec<u8>) -> Self {
        VirtualMachine { ip: 0, sp: 0, fp: 0, flag_eq: false, flag_gt: false, reg: [Variable::U8(0); 8], code: c, stack: Vec::new(), data: Vec::new(), is_executing: false } 
    }
    fn decode_variable(&mut self) -> Variable {
        self.ip += 1;
        match self.code[self.ip] {
            0 => {
                self.ip += 1;
                Variable::U8(self.code[self.ip] as u8)
            },
            1 => {
                self.ip += 1;
                Variable::I8(self.code[self.ip] as i8)
            },
            2 => {
                self.ip += 1;
                let size = mem::size_of::<u16>();
                let value = u16::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Variable::U16(value)
            },
            3 => {
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = i16::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Variable::I16(value)
            },
            4 => {
                self.ip += 1;
                let size = mem::size_of::<u32>();
                let value = u32::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Variable::U32(value)
            },
            5 => {
                self.ip += 1;
                let size = mem::size_of::<i32>();
                let value = i32::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Variable::I32(value)
            },
            6 => {
                self.ip += 1;
                let size = mem::size_of::<u64>();
                let value = u64::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Variable::U64(value)
            },
            7 => {
                self.ip += 1;
                let size = mem::size_of::<i64>();
                let value = i64::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Variable::I64(value)
            },
            8 => {
                self.ip += 1;
                let size = mem::size_of::<f32>();
                let value = f32::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Variable::F32(value)
            },
            9 => {
                self.ip += 1;
                let size = mem::size_of::<f64>();
                let value = f64::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += size - 1;
                Variable::F64(value)
            },
            _ => Variable::None()
        }
    }

    fn decode(&mut self) -> Instruction {
        match self.code[self.ip] {
            0 => Instruction::NOP(),
            1 => {
                self.ip += 1;
                let register = self.code[self.ip] as Register;
                let var = self.decode_variable();
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
                let var = self.decode_variable();
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
                let var = self.decode_variable();
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
            _ => Instruction::NOP(),
        }
    }

    fn execute(&mut self, instr: Instruction) -> bool
    {
        println!("Executing: {:?} ", instr);
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
                    Variable::U8(v) => {
                        self.ip = v as Address;
                    true
                    }
                    Variable::U16(v) => {
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
                match (v1, v2){
                    (Variable::U8(v), Variable::U8(u)) => {
                        self.flag_eq = v == u;
                        self.flag_gt = v > u;
                    },
                    (Variable::I8(v), Variable::I8(u)) => {
                        self.flag_eq = v == u;
                        self.flag_gt = v > u;
                    },
                    (Variable::U16(v), Variable::U16(u)) => {
                        self.flag_eq = v == u;
                        self.flag_gt = v > u;
                    },
                    (Variable::I16(v), Variable::I16(u)) => {
                        self.flag_eq = v == u;
                        self.flag_gt = v > u;
                    },
                    (Variable::U32(v), Variable::U32(u)) => {
                        self.flag_eq = v == u;
                        self.flag_gt = v > u;
                    },
                    (Variable::I32(v), Variable::I32(u)) => {
                        self.flag_eq = v == u;
                        self.flag_gt = v > u;
                    },
                    (Variable::U64(v), Variable::U64(u)) => {
                        self.flag_eq = v == u;
                        self.flag_gt = v > u;
                    },
                    (Variable::I64(v), Variable::I64(u)) => {
                        self.flag_eq = v == u;
                        self.flag_gt = v > u;
                    },
                    (Variable::F32(v), Variable::F32(u)) => {
                        self.flag_eq = v == u;
                        self.flag_gt = v > u;
                    },
                    (Variable::F64(v), Variable::F64(u)) => {
                        self.flag_eq = v == u;
                        self.flag_gt = v > u;
                    },
                    _ => {
                        self.flag_eq = false;
                    }
                }
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
                match (v1, v2){
                    (Variable::U8(v), Variable::U8(u)) => {
                        self.stack.push(Variable::U8(u+v));
                    },
                    (Variable::I8(v), Variable::I8(u)) => {
                        self.stack.push(Variable::I8(u+v));
                    },
                    (Variable::U16(v), Variable::U16(u)) => {
                        self.stack.push(Variable::U16(u+v));
                    },
                    (Variable::I16(v), Variable::I16(u)) => {
                        self.stack.push(Variable::I16(u+v));
                    },
                    (Variable::U32(v), Variable::U32(u)) => {
                        self.stack.push(Variable::U32(u+v));
                    },
                    (Variable::I32(v), Variable::I32(u)) => {
                        self.stack.push(Variable::I32(u+v));
                    },
                    (Variable::U64(v), Variable::U64(u)) => {
                        self.stack.push(Variable::U64(u+v));
                    },
                    (Variable::I64(v), Variable::I64(u)) => {
                        self.stack.push(Variable::I64(u+v));
                    },
                    (Variable::F32(v), Variable::F32(u)) => {
                        self.stack.push(Variable::F32(u+v));
                    },
                    (Variable::F64(v), Variable::F64(u)) => {
                        self.stack.push(Variable::F64(u+v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::SUB(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Variable::U8(v), Variable::U8(u)) => {
                        self.stack.push(Variable::U8(u-v));
                    },
                    (Variable::I8(v), Variable::I8(u)) => {
                        self.stack.push(Variable::I8(u-v));
                    },
                    (Variable::U16(v), Variable::U16(u)) => {
                        self.stack.push(Variable::U16(u-v));
                    },
                    (Variable::I16(v), Variable::I16(u)) => {
                        self.stack.push(Variable::I16(u-v));
                    },
                    (Variable::U32(v), Variable::U32(u)) => {
                        self.stack.push(Variable::U32(u-v));
                    },
                    (Variable::I32(v), Variable::I32(u)) => {
                        self.stack.push(Variable::I32(u-v));
                    },
                    (Variable::U64(v), Variable::U64(u)) => {
                        self.stack.push(Variable::U64(u-v));
                    },
                    (Variable::I64(v), Variable::I64(u)) => {
                        self.stack.push(Variable::I64(u-v));
                    },
                    (Variable::F32(v), Variable::F32(u)) => {
                        self.stack.push(Variable::F32(u-v));
                    },
                    (Variable::F64(v), Variable::F64(u)) => {
                        self.stack.push(Variable::F64(u-v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::MUL(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Variable::U8(v), Variable::U8(u)) => {
                        self.stack.push(Variable::U8(u*v));
                    },
                    (Variable::I8(v), Variable::I8(u)) => {
                        self.stack.push(Variable::I8(u*v));
                    },
                    (Variable::U16(v), Variable::U16(u)) => {
                        self.stack.push(Variable::U16(u*v));
                    },
                    (Variable::I16(v), Variable::I16(u)) => {
                        self.stack.push(Variable::I16(u*v));
                    },
                    (Variable::U32(v), Variable::U32(u)) => {
                        self.stack.push(Variable::U32(u*v));
                    },
                    (Variable::I32(v), Variable::I32(u)) => {
                        self.stack.push(Variable::I32(u*v));
                    },
                    (Variable::U64(v), Variable::U64(u)) => {
                        self.stack.push(Variable::U64(u*v));
                    },
                    (Variable::I64(v), Variable::I64(u)) => {
                        self.stack.push(Variable::I64(u*v));
                    },
                    (Variable::F32(v), Variable::F32(u)) => {
                        self.stack.push(Variable::F32(u*v));
                    },
                    (Variable::F64(v), Variable::F64(u)) => {
                        self.stack.push(Variable::F64(u*v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::DIV(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Variable::U8(v), Variable::U8(u)) => {
                        self.stack.push(Variable::U8(u/v));
                    },
                    (Variable::I8(v), Variable::I8(u)) => {
                        self.stack.push(Variable::I8(u/v));
                    },
                    (Variable::U16(v), Variable::U16(u)) => {
                        self.stack.push(Variable::U16(u/v));
                    },
                    (Variable::I16(v), Variable::I16(u)) => {
                        self.stack.push(Variable::I16(u/v));
                    },
                    (Variable::U32(v), Variable::U32(u)) => {
                        self.stack.push(Variable::U32(u/v));
                    },
                    (Variable::I32(v), Variable::I32(u)) => {
                        self.stack.push(Variable::I32(u/v));
                    },
                    (Variable::U64(v), Variable::U64(u)) => {
                        self.stack.push(Variable::U64(u/v));
                    },
                    (Variable::I64(v), Variable::I64(u)) => {
                        self.stack.push(Variable::I64(u/v));
                    },
                    (Variable::F32(v), Variable::F32(u)) => {
                        self.stack.push(Variable::F32(u/v));
                    },
                    (Variable::F64(v), Variable::F64(u)) => {
                        self.stack.push(Variable::F64(u/v));
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
                self.stack.push(Variable::U16(self.ip as u16 + 1));
                self.execute(Instruction::JMP(reg))
            },
            Instruction::RET() => {
                match self.stack.pop() {
                    Some(v) => {
                        match v {
                            Variable::U8(u) => {
                                self.ip = u as usize;
                                true
                            }
                            Variable::U16(u) => {
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
            _ => true
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

fn main() {
    let mut vm = VirtualMachine::new(vec![1, 0, 8, 0, 0, 0xc0, 0x3f, 1, 2, 8, 0, 0, 0xc0, 0x3f, 11, 0, 2, 19, 0, 7, 0, 22]);
    vm.cpu();
}
