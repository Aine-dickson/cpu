// Virtual CPU implementation

use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
struct Flags {
    carry: bool,
    zero: bool,
    interrupt: bool,
    sign: bool,
    overflow: bool,
    parity: bool,
}

#[derive(Copy, Clone)]
enum Register{
    AX(u8, u8),
    BX(u8, u8),
    CX(u8, u8),
    DX(u8, u8),
    EAX(u8, u8, u8, u8),
    EBX(u8, u8, u8, u8),
    ECX(u8, u8, u8, u8),
    EDX(u8, u8, u8, u8)
}



impl Register {
    fn flush(&mut self){
        match self {
            Register::AX(_, _) => {
                *self = Register::AX(0, 0);
            },
            Register::BX(_, _) => {
                *self = Register::BX(0, 0);
            },
            Register::CX(_, _) => {
                *self = Register::CX(0, 0);
            },
            Register::DX(_, _) => {
                *self = Register::DX(0, 0);
            },
            Register::EAX(_, _, _, _) => {
                *self = Register::EAX(0, 0, 0, 0);
            },
            Register::EBX(_, _, _, _) => {
                *self = Register::EBX(0, 0, 0, 0);
            },
            Register::ECX(_, _, _, _) => {
                *self = Register::ECX(0, 0, 0, 0);
            },
            Register::EDX(_, _, _, _) => {
                *self = Register::EDX(0, 0, 0, 0);
            },
        }
    }
}

impl Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::AX(a, b) => write!(f, "AX: {:?}{:?}", a, b),
            Register::BX(a, b) => write!(f, "BX: {:?}{:?}", a, b),
            Register::CX(a, b) => write!(f, "CX: {:?}{:?}", a, b),
            Register::DX(a, b) => write!(f, "DX: {:?}{:?}", a, b),
            Register::EAX(a, b, c, d) => write!(f, "EAX: {:?}{:?}{:?}{:?}", a, b, c, d),
            Register::EBX(a, b, c, d) => write!(f, "EBX: {:?}{:?}{:?}{:?}", a, b, c, d),
            Register::ECX(a, b, c, d) => write!(f, "ECX: {:?}{:?}{:?}{:?}", a, b, c, d),
            Register::EDX(a, b, c, d) => write!(f, "EDX: {:?}{:?}{:?}{:?}", a, b, c, d),
        }
    }
}

impl PartialEq for Register {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Register::AX(_, _), Register::AX(_, _)) => true,
            (Register::BX(_, _), Register::BX(_, _)) => true,
            (Register::CX(_, _), Register::CX(_, _)) => true,
            (Register::DX(_, _), Register::DX(_, _)) => true,
            (Register::EAX(_, _, _, _), Register::EAX(_, _, _, _)) => true,
            (Register::EBX(_, _, _, _), Register::EBX(_, _, _, _)) => true,
            (Register::ECX(_, _, _, _), Register::ECX(_, _, _, _)) => true,
            (Register::EDX(_, _, _, _), Register::EDX(_, _, _, _)) => true,
            _ => false
        }
    }    
}

#[derive(Debug, Clone)]
struct Instruction{
    opcode: IS,
    operands: Vec<Operand>,
}

#[derive(Debug, Clone)]
enum IS {
    Mov, Add, Sub, Mul,
    Div, And, Or, Xor,
    Not, Cmp, Jmp, Je,
    Jne, Jg, Jge, Jl,
    Jle, Call, Ret, Push, 
    Pop
}

#[derive(Debug, Clone)]
enum Operand{
    Register(Register),
    Memory(String),
    Immediate(Data),
}

#[derive(Debug, Clone)]
enum Data{
    Byte(u8),
    Word(u16),
    Dword(u32),
}

impl Data {
    fn get_value(&self) -> usize{
        match self {
            Data::Byte(value) => *value as usize,
            Data::Word(value) => *value as usize,
            Data::Dword(value) => *value as usize,
        }
    }
}

#[derive(Debug)]
struct Program{
    data_section: HashMap<String, Data>,
    bss_section: HashMap<String, Data>,
    text_section: Vec<Instruction>,
}

struct CPU{
    registers: HashMap<String, Register>,
    memory: Program,
    stack: Vec<u8>,
    program_counter: usize,
    flags: Flags
}

impl Debug  for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Internal CPU state: 

registers: {:#?},

memory: {:?}, 
stack: {:?}, 
flags: {:?}", self.registers, self.memory, self.stack, self.flags)
    }
    
}

impl CPU {
    fn new(program: Program)-> Self{
        Self {
            registers: HashMap::from([
                ("AX".to_owned(), Register::AX(0, 0)),
                ("BX".to_owned(), Register::BX(0, 0)),
                ("CX".to_owned(), Register::CX(0, 0)),
                ("DX".to_owned(), Register::DX(0, 0)),
                ("EAX".to_owned(), Register::EAX(0, 0, 0, 0)),
                ("EBX".to_owned(), Register::EBX(0, 0, 0, 0)),
                ("ECX".to_owned(), Register::ECX(0, 0, 0, 0)),
                ("EDX".to_owned(), Register::EDX(0, 0, 0, 0)),
            ]),
            memory: program,
            stack: Vec::new(),
            program_counter: 0,
            flags: Flags{carry: false, zero: false, interrupt: false, sign: false, overflow: false, parity: false}
        }
    }

    fn display_state(self){
        println!("{:?}", self)
    }

    fn display_registers(&self){
        println!("Registers' state:\n");
        self.registers.iter().for_each(|(_, value)|{
            println!("{:>10?}",value);
        });
    }

    fn load_program(&mut self, program: Program){
        self.memory = program;
    }

    fn fetch(&mut self) -> Option<Instruction>{
        println!("Program has {} instructions", self.memory.text_section.len());
        if self.program_counter >= self.memory.text_section.len(){
            return None;
        }
        self.program_counter += 1;
        Some(self.memory.text_section[self.program_counter].clone())
    }

    fn run(&mut self){
        loop {
            self.execute();

            if self.program_counter >= self.memory.text_section.len(){
                break;
            }
        }
    }

    fn stop(&mut self){
        self.program_counter = self.memory.text_section.len();
    }

    fn mov(&mut self, source: &Operand, destination: &Operand){
        match (destination, source) {
            (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
                match (dest_reg, src_reg) {
                    (Register::AX(_, _), Register::BX(_, _)) => {
                        let bx = self.registers.insert("BX".to_owned(), Register::BX(0, 0)).unwrap();
                        self.registers.insert("AX".to_owned(), bx);
                    },
                    (Register::AX(_, _), Register::CX(_, _)) => {
                        let cx = self.registers.insert("CX".to_owned(), Register::CX(0, 0)).unwrap();
                        self.registers.insert("AX".to_owned(), cx);
                    },
                    (Register::AX(_, _), Register::DX(_, _)) => {
                        let dx = self.registers.insert("DX".to_owned(), Register::DX(0, 0)).unwrap();
                        self.registers.insert("AX".to_owned(), dx);
                    },
                    (Register::AX(_, _), Register::EAX(_, _, _, _)) => {
                        if let Register::EAX(a, b, c, d) = self.registers.get("EAX").unwrap() {
                            let eax = Register::EAX(0, 0, *c, *d);
                            let ax = Register::AX(*a, *b);
                            self.registers.insert("EAX".to_owned(), eax);
                            self.registers.insert("AX".to_owned(), ax);
                        }
                    },
                    (Register::AX(_, _), Register::EBX(_, _, _, _)) => {
                        if let Register::EBX(a, b, c, d) = self.registers.get("EBX").unwrap() {
                            let ebx = Register::EBX(0, 0, *c, *d);
                            let ax = Register::AX(*a, *b);
                            self.registers.insert("EBX".to_owned(), ebx);
                            self.registers.insert("AX".to_owned(), ax);
                        }
                    },
                    (Register::AX(_, _), Register::ECX(_, _, _, _)) => {
                        if let Register::ECX(a, b, c, d) = self.registers.get("ECX").unwrap() {
                            let ecx = Register::ECX(0, 0, *c, *d);
                            let ax = Register::AX(*a, *b);
                            self.registers.insert("ECX".to_owned(), ecx);
                            self.registers.insert("AX".to_owned(), ax);
                        }
                    },
                    (Register::AX(_, _), Register::EDX(_, _, _, _)) => {
                        if let Register::EDX(a, b, c, d) = self.registers.get("EDX").unwrap() {
                            let edx = Register::EDX(0, 0, *c, *d);
                            let ax = Register::AX(*a, *b);
                            self.registers.insert("EDX".to_owned(), edx);
                            self.registers.insert("AX".to_owned(), ax);
                        }
                    },
                    (Register::BX(_, _), Register::AX(_, _)) => {
                        let ax = self.registers.insert("AX".to_owned(), Register::AX(0, 0)).unwrap();
                        self.registers.insert("BX".to_owned(), ax);
                    },
                    (Register::BX(_, _), Register::CX(_, _)) => {
                        let cx = self.registers.insert("CX".to_owned(), Register::CX(0, 0)).unwrap();
                        self.registers.insert("BX".to_owned(), cx);
                    },
                    (Register::BX(_, _), Register::DX(_, _)) => {
                        let dx = self.registers.insert("DX".to_owned(), Register::DX(0, 0)).unwrap();
                        self.registers.insert("BX".to_owned(), dx);
                    },
                    (Register::BX(_, _), Register::EAX(_, _, _, _)) => {
                        if let Register::EAX(a, b, c, d) = self.registers.get("EAX").unwrap() {
                            let eax = Register::EAX(0, 0, *c, *d);
                            let bx = Register::BX(*a, *b);
                            self.registers.insert("EAX".to_owned(), eax);
                            self.registers.insert("BX".to_owned(), bx);
                        }
                    },
                    (Register::BX(_, _), Register::EBX(_, _, _, _)) => {
                        if let Register::EBX(a, b, c, d) = self.registers.get("EBX").unwrap() {
                            let ebx = Register::EBX(0, 0, *c, *d);
                            let bx = Register::BX(*a, *b);
                            self.registers.insert("EBX".to_owned(), ebx);
                            self.registers.insert("BX".to_owned(), bx);
                        }
                    },
                    (Register::BX(_, _), Register::ECX(_, _, _, _)) => {
                        if let Register::ECX(a, b, c, d) = self.registers.get("ECX").unwrap() {
                            let ecx = Register::ECX(0, 0, *c, *d);
                            let bx = Register::BX(*a, *b);
                            self.registers.insert("ECX".to_owned(), ecx);
                            self.registers.insert("BX".to_owned(), bx);
                        }
                    },
                    (Register::BX(_, _), Register::EDX(_, _, _, _)) => {
                        if let Register::EDX(a, b, c, d) = self.registers.get("EDX").unwrap() {
                            let edx = Register::EDX(0, 0, *c, *d);
                            let bx = Register::BX(*a, *b);
                            self.registers.insert("EDX".to_owned(), edx);
                            self.registers.insert("BX".to_owned(), bx);
                        }
                    },
                    (Register::CX(_, _), Register::AX(_, _)) => {
                        let ax = self.registers.insert("AX".to_owned(), Register::AX(0, 0)).unwrap();
                        self.registers.insert("CX".to_owned(), ax);
                    },
                    (Register::CX(_, _), Register::BX(_, _)) => {
                        let bx = self.registers.insert("BX".to_owned(), Register::BX(0, 0)).unwrap();
                        self.registers.insert("CX".to_owned(), bx);
                    },
                    (Register::CX(_, _), Register::DX(_, _)) => {
                        let dx = self.registers.insert("DX".to_owned(), Register::DX(0, 0)).unwrap();
                        self.registers.insert("CX".to_owned(), dx);
                    },
                    (Register::CX(_, _), Register::EAX(_, _, _, _)) => {
                        if let Register::EAX(a, b, c, d) = self.registers.get("EAX").unwrap() {
                            let eax = Register::EAX(0, 0, *c, *d);
                            let cx = Register::CX(*a, *b);
                            self.registers.insert("EAX".to_owned(), eax);
                            self.registers.insert("CX".to_owned(), cx);
                        }
                    },
                    (Register::CX(_, _), Register::EBX(_, _, _, _)) => {
                        if let Register::EBX(a, b, c, d) = self.registers.get("EBX").unwrap() {
                            let ebx = Register::EBX(0, 0, *c, *d);
                            let cx = Register::CX(*a, *b);
                            self.registers.insert("EBX".to_owned(), ebx);
                            self.registers.insert("CX".to_owned(), cx);
                        }
                    },
                    (Register::CX(_, _), Register::ECX(_, _, _, _)) => {
                        if let Register::ECX(a, b, c, d) = self.registers.get("ECX").unwrap() {
                            let ecx = Register::ECX(0, 0, *c, *d);
                            let cx = Register::CX(*a, *b);
                            self.registers.insert("ECX".to_owned(), ecx);
                            self.registers.insert("CX".to_owned(), cx);
                        }
                    },
                    (Register::CX(_, _), Register::EDX(_, _, _, _)) => {
                        if let Register::EDX(a, b, c, d) = self.registers.get("EDX").unwrap() {
                            let edx = Register::EDX(0, 0, *c, *d);
                            let cx = Register::CX(*a, *b);
                            self.registers.insert("EDX".to_owned(), edx);
                            self.registers.insert("CX".to_owned(), cx);
                        }
                    },
                    (Register::DX(_, _), Register::AX(_, _)) => {
                        let ax = self.registers.insert("AX".to_owned(), Register::AX(0, 0)).unwrap();
                        self.registers.insert("DX".to_owned(), ax);
                    },
                    (Register::DX(_, _), Register::BX(_, _)) => {
                        let bx = self.registers.insert("BX".to_owned(), Register::BX(0, 0)).unwrap();
                        self.registers.insert("DX".to_owned(), bx);
                    },
                    (Register::DX(_, _), Register::CX(_, _)) => {
                        let cx = self.registers.insert("CX".to_owned(), Register::CX(0, 0)).unwrap();
                        self.registers.insert("DX".to_owned(), cx);
                    },
                    (Register::DX(_, _), Register::EAX(_, _, _, _)) => {
                        if let Register::EAX(a, b, c, d) = self.registers.get("EAX").unwrap() {
                            let eax = Register::EAX(0, 0, *c, *d);
                            let dx = Register::DX(*a, *b);
                            self.registers.insert("EAX".to_owned(), eax);
                            self.registers.insert("DX".to_owned(), dx);
                        }
                    },
                    (Register::DX(_, _), Register::EBX(_, _, _, _)) => {
                        if let Register::EBX(a, b, c, d) = self.registers.get("EBX").unwrap() {
                            let ebx = Register::EBX(0, 0, *c, *d);
                            let dx = Register::DX(*a, *b);
                            self.registers.insert("EBX".to_owned(), ebx);
                            self.registers.insert("DX".to_owned(), dx);
                        }
                    },
                    (Register::DX(_, _), Register::ECX(_, _, _, _)) => {
                        if let Register::ECX(a, b, c, d) = self.registers.get("ECX").unwrap() {
                            let ecx = Register::ECX(0, 0, *c, *d);
                            let dx = Register::DX(*a, *b);
                            self.registers.insert("ECX".to_owned(), ecx);
                            self.registers.insert("DX".to_owned(), dx);
                        }
                    },
                    (Register::DX(_, _), Register::EDX(_, _, _, _)) => {
                        if let Register::EDX(a, b, c, d) = self.registers.get("EDX").unwrap() {
                            let edx = Register::EDX(0, 0, *c, *d);
                            let dx = Register::DX(*a, *b);
                            self.registers.insert("EDX".to_owned(), edx);
                            self.registers.insert("DX".to_owned(), dx);
                        }
                    },
                    (Register::EAX(_, _, _, _), Register::AX(_, _)) => {
                        if let Register::AX(a, b) = self.registers.insert("AX".to_owned(), Register::AX(0, 0)).unwrap(){
                            if let Register::EAX(_, _, axl, axh) = self.registers.get("EAX").unwrap(){
                                let eax = Register::EAX(a, b, *axl, *axh);
                                self.registers.insert("EAX".to_owned(), eax);
                            }
                        };
                    },
                    (Register::EAX(_, _, _, _), Register::BX(_, _)) => {
                        if let Register::BX(a, b) = self.registers.insert("BX".to_owned(), Register::BX(0, 0)).unwrap(){
                            if let Register::ECX(_, _, axl, axh) = self.registers.get("ECX").unwrap(){
                                let ecx = Register::ECX(a, b, *axl, *axh);
                                self.registers.insert("ECX".to_owned(), ecx);
                            }
                        };
                    },
                    (Register::EAX(_, _, _, _), Register::CX(_, _)) => {
                        if let Register::CX(a, b) = self.registers.insert("CX".to_owned(), Register::CX(0, 0)).unwrap(){
                            if let Register::EAX(_, _, axl, axh) = self.registers.get("EAX").unwrap(){
                                let eax = Register::EAX(a, b, *axl, *axh);
                                self.registers.insert("EAX".to_owned(), eax);
                            }
                        };
                    },
                    (Register::EAX(_, _, _, _), Register::DX(_, _)) => {
                        if let Register::DX(a, b) = self.registers.insert("DX".to_owned(), Register::DX(0, 0)).unwrap(){
                            if let Register::EAX(_, _, axl, axh) = self.registers.get("EAX").unwrap(){
                                let eax = Register::EAX(a, b, *axl, *axh);
                                self.registers.insert("EAX".to_owned(), eax);
                            }
                        };
                    },
                    (Register::EAX(_, _, _, _), Register::EBX(_, _, _, _)) => {
                        if let Register::EBX(a, b, c, d) = self.registers.insert("EBX".to_owned(), Register::EBX(0, 0, 0, 0)).unwrap(){
                            let eax = Register::EAX(a, b, c, d);
                            self.registers.insert("EAX".to_owned(), eax);
                        };
                    },
                    (Register::EAX(_, _, _, _), Register::ECX(_, _, _, _)) => {
                        if let Register::ECX(a, b, c, d) = self.registers.insert("ECX".to_owned(), Register::ECX(0, 0, 0, 0)).unwrap(){
                            let eax = Register::EAX(a, b, c, d);
                            self.registers.insert("EAX".to_owned(), eax);
                        };
                    },
                    (Register::EAX(_, _, _, _), Register::EDX(_, _, _, _)) => {
                        if let Register::EDX(a, b, c, d) = self.registers.insert("EDX".to_owned(), Register::EDX(0, 0, 0, 0)).unwrap(){
                            let eax = Register::EAX(a, b, c, d);
                            self.registers.insert("EAX".to_owned(), eax);
                        };
                    },
                    (Register::EBX(_, _, _, _), Register::AX(_, _)) => {
                        if let Register::AX(a, b) = self.registers.insert("AX".to_owned(), Register::AX(0, 0)).unwrap(){
                            if let Register::EBX(_, _, bxl, bxh) = self.registers.get("EBX").unwrap(){
                                let ebx = Register::EBX(a, b, *bxl, *bxh);
                                self.registers.insert("EBX".to_owned(), ebx);
                            }
                        };
                    },
                    (Register::EBX(_, _, _, _), Register::BX(_, _)) => {
                        if let Register::BX(a, b) = self.registers.insert("BX".to_owned(), Register::BX(0, 0)).unwrap(){
                            if let Register::EDX(_, _, dxl, dxh) = self.registers.get("EDX").unwrap(){
                                let edx = Register::EDX(a, b, *dxl, *dxh);
                                self.registers.insert("EDX".to_owned(), edx);
                            }
                        };
                    },
                    (Register::EBX(_, _, _, _), Register::CX(_, _)) => {
                        if let Register::CX(a, b) = self.registers.insert("CX".to_owned(), Register::CX(0, 0)).unwrap(){
                            if let Register::EBX(_, _, bxl, bxh) = self.registers.get("EBX").unwrap(){
                                let ebx = Register::EBX(a, b, *bxl, *bxh);
                                self.registers.insert("EBX".to_owned(), ebx);
                            }
                        };
                    },
                    (Register::EBX(_, _, _, _), Register::DX(_, _)) => {
                        if let Register::DX(a, b) = self.registers.insert("DX".to_owned(), Register::DX(0, 0)).unwrap(){
                            if let Register::EBX(_, _, bxl, bxh) = self.registers.get("EBX").unwrap(){
                                let ebx = Register::EBX(a, b, *bxl, *bxh);
                                self.registers.insert("EBX".to_owned(), ebx);
                            }
                        };
                    },
                    (Register::EBX(_, _, _, _), Register::EAX(_, _, _, _)) => {
                        if let Register::EAX(a, b, c, d) = self.registers.insert("EAX".to_owned(), Register::EAX(0, 0, 0, 0)).unwrap(){
                            let ebx = Register::EBX(a, b, c, d);
                            self.registers.insert("EBX".to_owned(), ebx);
                        };
                    },
                    (Register::EBX(_, _, _, _), Register::ECX(_, _, _, _)) => {
                        if let Register::ECX(a, b, c, d) = self.registers.insert("ECX".to_owned(), Register::ECX(0, 0, 0, 0)).unwrap(){
                            let ebx = Register::EBX(a, b, c, d);
                            self.registers.insert("EBX".to_owned(), ebx);
                        };
                    },
                    (Register::EBX(_, _, _, _), Register::EDX(_, _, _, _)) => {
                        if let Register::EDX(a, b, c, d) = self.registers.insert("EDX".to_owned(), Register::EDX(0, 0, 0, 0)).unwrap(){
                            let ebx = Register::EBX(a, b, c, d);
                            self.registers.insert("EBX".to_owned(), ebx);
                        };
                    },
                    (Register::ECX(_, _, _, _), Register::AX(_, _)) => {
                        if let Register::AX(a, b) = self.registers.insert("AX".to_owned(), Register::AX(0, 0)).unwrap(){
                            if let Register::ECX(_, _, cxl, cxh) = self.registers.get("ECX").unwrap(){
                                let edx = Register::ECX(a, b, *cxl, *cxh);
                                self.registers.insert("ECX".to_owned(), edx);
                            }
                        };
                    },
                    (Register::ECX(_, _, _, _), Register::BX(_, _)) => {
                        if let Register::BX(a, b) = self.registers.insert("BX".to_owned(), Register::BX(0, 0)).unwrap(){
                            if let Register::ECX(_, _, cxl, cxh) = self.registers.get("ECX").unwrap(){
                                let edx = Register::ECX(a, b, *cxl, *cxh);
                                self.registers.insert("ECX".to_owned(), edx);
                            }
                        };
                    },
                    (Register::ECX(_, _, _, _), Register::CX(_, _)) => {
                        if let Register::CX(a, b) = self.registers.insert("CX".to_owned(), Register::CX(0, 0)).unwrap(){
                            if let Register::EDX(_, _, cxl, cxh) = self.registers.get("ECX").unwrap(){
                                let edx = Register::ECX(a, b, *cxl, *cxh);
                                self.registers.insert("EDX".to_owned(), edx);
                            }
                        };
                    },
                    (Register::ECX(_, _, _, _), Register::DX(_, _)) => {
                        if let Register::DX(a, b) = self.registers.insert("DX".to_owned(), Register::DX(0, 0)).unwrap(){
                            if let Register::ECX(_, _, cxl, cxh) = self.registers.get("ECX").unwrap(){
                                let edx = Register::ECX(a, b, *cxl, *cxh);
                                self.registers.insert("ECX".to_owned(), edx);
                            }
                        };
                    },
                    (Register::ECX(_, _, _, _), Register::EAX(_, _, _, _)) => {
                        if let Register::EAX(a, b, c, d) = self.registers.insert("EAX".to_owned(), Register::EAX(0, 0, 0, 0)).unwrap(){
                            let ecx = Register::ECX(a, b, c, d);
                            self.registers.insert("ECX".to_owned(), ecx);
                        };
                    },
                    (Register::ECX(_, _, _, _), Register::EBX(_, _, _, _)) => {
                        if let Register::EBX(a, b, c, d) = self.registers.insert("EBX".to_owned(), Register::EBX(0, 0, 0, 0)).unwrap(){
                            let ecx = Register::ECX(a, b, c, d);
                            self.registers.insert("ECX".to_owned(), ecx);
                        };
                    },
                    (Register::ECX(_, _, _, _), Register::EDX(_, _, _, _)) => {
                        if let Register::EDX(a, b, c, d) = self.registers.insert("EDX".to_owned(), Register::EDX(0, 0, 0, 0)).unwrap(){
                            let ecx = Register::ECX(a, b, c, d);
                            self.registers.insert("ECX".to_owned(), ecx);
                        };
                    },
                    (Register::EDX(_, _, _, _), Register::AX(_, _)) => {
                        if let Register::AX(a, b) = self.registers.insert("AX".to_owned(), Register::AX(0, 0)).unwrap(){
                            if let Register::EDX(_, _, dxl, dxh) = self.registers.get("EDX").unwrap(){
                                let edx = Register::EDX(a, b, *dxl, *dxh);
                                self.registers.insert("EDX".to_owned(), edx);
                            }
                        };
                    },
                    (Register::EDX(_, _, _, _), Register::BX(_, _)) => {
                        if let Register::BX(a, b) = self.registers.insert("BX".to_owned(), Register::BX(0, 0)).unwrap(){
                            if let Register::EDX(_, _, dxl, dxh) = self.registers.get("EDX").unwrap(){
                                let edx = Register::EDX(a, b, *dxl, *dxh);
                                self.registers.insert("EDX".to_owned(), edx);
                            }
                        };
                    },
                    (Register::EDX(_, _, _, _), Register::CX(_, _)) => {
                        if let Register::CX(a, b) = self.registers.insert("CX".to_owned(), Register::CX(0, 0)).unwrap(){
                            if let Register::EDX(_, _, dxl, dxh) = self.registers.get("EDX").unwrap(){
                                let edx = Register::EDX(a, b, *dxl, *dxh);
                                self.registers.insert("EDX".to_owned(), edx);
                            }
                        };
                    },
                    (Register::EDX(_, _, _, _), Register::DX(_, _)) => {
                        if let Register::DX(a, b) = self.registers.insert("DX".to_owned(), Register::DX(0, 0)).unwrap(){
                            if let Register::EDX(_, _, dxl, dxh) = self.registers.get("EDX").unwrap(){
                                let edx = Register::EDX(a, b, *dxl, *dxh);
                                self.registers.insert("EDX".to_owned(), edx);
                            }
                        };
                    },
                    (Register::EDX(_, _, _, _), Register::EAX(_, _, _, _)) => {
                        if let Register::EAX(a, b, c, d) = self.registers.insert("EAX".to_owned(), Register::EAX(0, 0, 0, 0)).unwrap(){
                            let edx = Register::EDX(a, b, c, d);
                            self.registers.insert("EDX".to_owned(), edx);
                        };
                    },
                    (Register::EDX(_, _, _, _), Register::EBX(_, _, _, _)) => {
                        if let Register::EBX(a, b, c, d) = self.registers.insert("EBX".to_owned(), Register::EBX(0, 0, 0, 0)).unwrap(){
                            let edx = Register::EDX(a, b, c, d);
                            self.registers.insert("EDX".to_owned(), edx);
                        };
                    },
                    (Register::EDX(_, _, _, _), Register::ECX(_, _, _, _)) => {
                        if let Register::ECX(a, b, c, d) = self.registers.insert("ECX".to_owned(), Register::ECX(0, 0, 0, 0)).unwrap(){
                            let edx = Register::EDX(a, b, c, d);
                            self.registers.insert("EDX".to_owned(), edx);
                        };
                    },
                    _ => {
                        println!("Skipping redundant move operation Mov {:?},{:?}. Rsn: Source and destination are equal", destination, source);
                        return;
                    }
                }
            },
            (Operand::Register(register), Operand::Memory(address)) => {
                let address = match self.memory.data_section.contains_key(address) {
                    true => {
                        address
                    },
                    false => {
                        match self.memory.bss_section.contains_key(address) {
                            true => {
                                address
                            },
                            false => {
                                println!("Use of undeclared value address {} at instruction Mov {:?}, {}", address, register, address);
                                panic!("Program terminated due to invalid memory access");
                            }
                        }
                    }   
                };
                match (register, address) {
                    (Register::AX(_, _), _) => {
                        let data = self.memory.data_section.get(address).unwrap();
                        if data.get_value() == 0 {
                            println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                            panic!("Program terminated due to invalid memory access");
                        }
                        
                        match data {
                            Data::Byte(value) => {
                                if let Register::AX(_, ah) = self.registers.get_mut("AX").unwrap(){
                                    *ah = *value;
                                }
                            },
                            Data::Word(value) => {
                                let bytes = value.to_le_bytes();
                                if let Register::AX(al, ah) = self.registers.get_mut("AX").unwrap(){
                                    *al = bytes[0];
                                    *ah = bytes[1];
                                }
                            },
                            Data::Dword(value) => {
                                println!("Data type mismatch. Expected byte or word, found dword at address {} for instruction Mov {:?}, {}\nThis leads data truncation/loss", address, register, address);
                                let bytes = value.to_le_bytes();
                                if let Register::AX(al, ah) = self.registers.get_mut("AX").unwrap(){
                                    *al = bytes[0];
                                    *ah = bytes[1];
                                }
                            },
                        }
                    },
                    (Register::BX(_, _), _) => {
                        let data = self.memory.data_section.get(address).unwrap();
                        if data.get_value() == 0 {
                            println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                            panic!("Program terminated due to invalid memory access");
                        }
                        match data {
                            Data::Byte(value) => {
                                if let Register::BX(_, bh) = self.registers.get_mut("BX").unwrap(){
                                    *bh = *value;
                                }
                            },
                            Data::Word(value) => {
                                let bytes = value.to_le_bytes();
                                if let Register::BX(bl, bh) = self.registers.get_mut("BX").unwrap(){
                                    *bl = bytes[0];
                                    *bh = bytes[1];
                                }
                            },
                            Data::Dword(value) => {
                                println!("Data type mismatch. Expected byte or word, found dword at address {} for instruction Mov {:?}, {}\nThis leads data truncation/loss", address, register, address);
                                let bytes = value.to_le_bytes();
                                if let Register::BX(bl, bh) = self.registers.get_mut("BX").unwrap(){
                                    *bl = bytes[0];
                                    *bh = bytes[1];
                                }
                            },
                        }
                    },
                    (Register::CX(_, _), _) => {
                        let data = self.memory.data_section.get(address).unwrap();
                        if data.get_value() == 0 {
                            println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                            panic!("Program terminated due to invalid memory access");
                        }

                        match data {
                            Data::Byte(value) => {
                                if let Register::CX(_, ch) = self.registers.get_mut("CX").unwrap(){
                                    *ch = *value;
                                }
                            },
                            Data::Word(value) => {
                                let bytes = value.to_le_bytes();
                                if let Register::CX(cl, ch) = self.registers.get_mut("CX").unwrap(){
                                    *cl = bytes[0];
                                    *ch = bytes[1];
                                }
                            },
                            Data::Dword(value) => {
                                println!("Data type mismatch. Expected byte or word, found dword at address {} for instruction Mov {:?}, {}\nThis leads data truncation/loss", address, register, address);
                                let bytes = value.to_le_bytes();
                                if let Register::CX(cl, ch) = self.registers.get_mut("CX").unwrap(){
                                    *cl = bytes[0];
                                    *ch = bytes[1];
                                }
                            },
                        }
                    },
                    (Register::DX(_, _), _) => {
                        let data = self.memory.data_section.get(address).unwrap();
                        if data.get_value() == 0 {
                            println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                            panic!("Program terminated due to invalid memory access");
                        }
                        match data {
                            Data::Byte(value) => {
                                if let Register::DX(_, dh) = self.registers.get_mut("DX").unwrap(){
                                    *dh = *value;
                                }
                            },
                            Data::Word(value) => {
                                let bytes = value.to_le_bytes();
                                if let Register::DX(dl, dh) = self.registers.get_mut("DX").unwrap(){
                                    *dl = bytes[0];
                                    *dh = bytes[1];
                                }
                            },
                            Data::Dword(value) => {
                                println!("Data type mismatch. Expected byte or word, found dword at address {} for instruction Mov {:?}, {}\nThis leads data truncation/loss", address, register, address);
                                let bytes = value.to_le_bytes();
                                if let Register::DX(dl, dh) = self.registers.get_mut("DX").unwrap(){
                                    *dl = bytes[0];
                                    *dh = bytes[1];
                                }
                            },
                        }
                    },
                    (Register::EAX(_, _, _, _), _) => {
                        let data = self.memory.data_section.get(address).unwrap();
                        if data.get_value() == 0 {
                            println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                            panic!("Program terminated due to invalid memory access");
                        }

                        match data {
                            Data::Byte(value) => {
                                if let Register::EAX(al, _, _, _) = self.registers.get_mut("EAX").unwrap(){
                                    *al = *value;
                                }
                            },
                            Data::Word(value) => {
                                let bytes = value.to_le_bytes();
                                if let Register::EAX(al, ah, _, _) = self.registers.get_mut("EAX").unwrap(){
                                    *al = bytes[0];
                                    *ah = bytes[1];
                                }
                            },
                            Data::Dword(value) => {
                                let bytes = value.to_le_bytes();
                                if let Register::EAX(al, ah, axl, axh) = self.registers.get_mut("EAX").unwrap(){
                                    *al = bytes[0];
                                    *ah = bytes[1];
                                    *axl = bytes[2];
                                    *axh = bytes[3];
                                }
                            },
                        }
                    },
                    (Register::EBX(_, _, _, _), _) => {
                        let data = self.memory.data_section.get(address).unwrap();
                        if data.get_value() == 0 {
                            println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                            panic!("Program terminated due to invalid memory access");
                        }

                        match data {
                            Data::Byte(value) => {
                                if let Register::EBX(bl, _, _, _) = self.registers.get_mut("EBX").unwrap(){
                                    *bl = *value;
                                }
                            },
                            Data::Word(value) => {
                                let bytes = value.to_le_bytes();
                                if let Register::EBX(bl, bh, _, _) = self.registers.get_mut("EBX").unwrap(){
                                    *bl = bytes[0];
                                    *bh = bytes[1];
                                }
                            },
                            Data::Dword(value) => {
                                let bytes = value.to_le_bytes();
                                if let Register::EBX(bl, bh, bxl, bxh) = self.registers.get_mut("EBX").unwrap(){
                                    *bl = bytes[0];
                                    *bh = bytes[1];
                                    *bxl = bytes[2];
                                    *bxh = bytes[3];
                                }
                            },
                        }
                    },
                    (Register::ECX(_, _, _, _), _) => {
                        let data = self.memory.data_section.get(address).unwrap();
                        if data.get_value() == 0 {
                            println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                            panic!("Program terminated due to invalid memory access");
                        }

                        match data {
                            Data::Byte(value) => {
                                if let Register::ECX(cl, _, _, _) = self.registers.get_mut("ECX").unwrap(){
                                    *cl = *value;
                                }
                            },
                            Data::Word(value) => {
                                let bytes = value.to_le_bytes();
                                if let Register::ECX(cl, ch, _, _) = self.registers.get_mut("ECX").unwrap(){
                                    *cl = bytes[0];
                                    *ch = bytes[1];
                                }
                            },
                            Data::Dword(value) => {
                                let bytes = value.to_le_bytes();
                                if let Register::ECX(cl, ch, cxl, cxh) = self.registers.get_mut("ECX").unwrap(){
                                    *cl = bytes[0];
                                    *ch = bytes[1];
                                    *cxl = bytes[2];
                                    *cxh = bytes[3];
                                }
                            },
                        }
                    },
                    (Register::EDX(_, _, _, _), _) => {
                        let data = self.memory.data_section.get(address).unwrap();
                        if data.get_value() == 0 {
                            println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                            panic!("Program terminated due to invalid memory access");
                        }

                        match data {
                            Data::Byte(value) => {
                                if let Register::EDX(dl, _, _, _) = self.registers.get_mut("EDX").unwrap(){
                                    *dl = *value;
                                }
                            },
                            Data::Word(value) => {
                                let bytes = value.to_le_bytes();
                                if let Register::EDX(dl, dh, _, _) = self.registers.get_mut("EDX").unwrap(){
                                    *dl = bytes[0];
                                    *dh = bytes[1];
                                }
                            },
                            Data::Dword(value) => {
                                let bytes = value.to_le_bytes();
                                if let Register::EDX(dl, dh, dxl, dxh) = self.registers.get_mut("EDX").unwrap(){
                                    *dl = bytes[0];
                                    *dh = bytes[1];
                                    *dxl = bytes[2];
                                    *dxh = bytes[3];
                                }
                            },
                        }
                    },
                }
            },
            (Operand::Register(register), Operand::Immediate(value)) => {
                match register {
                    Register::AX(_, _) => {
                        match value {
                            Data::Byte(data) => {
                                if let Register::AX(al, _) = self.registers.get_mut("AX").unwrap(){
                                    *al = *data;
                                }
                            },
                            Data::Word(data) => {
                                let bytes = data.to_le_bytes();
                                if let Register::AX(al, ah) = self.registers.get_mut("AX").unwrap(){
                                    *al = bytes[0];
                                    *ah = bytes[1];
                                }
                            },
                            Data::Dword(data) => {
                                println!("Data type mismatch. Expected byte or word, found dword for instruction Mov {:?}, {}\nThis leads data truncation/loss", register, data);
                                let bytes = data.to_le_bytes();
                                if let Register::AX(al, ah) = self.registers.get_mut("AX").unwrap(){
                                    *al = bytes[0];
                                    *ah = bytes[1];
                                }
                            },
                        }
                    },
                    Register::BX(_, _) => {
                        match value {
                            Data::Byte(data) => {
                                if let Register::BX(bl, _) = self.registers.get_mut("BX").unwrap(){
                                    *bl = *data;
                                }
                            },
                            Data::Word(data) => {
                                let bytes = data.to_le_bytes();
                                if let Register::BX(bl, bh) = self.registers.get_mut("BX").unwrap(){
                                    *bl = bytes[0];
                                    *bh = bytes[1];
                                }
                            },
                            Data::Dword(data) => {
                                println!("Data type mismatch. Expected byte or word, found dword for instruction Mov {:?}, {}\nThis leads data truncation/loss", register, data);
                                let bytes = data.to_le_bytes();
                                if let Register::BX(bl, bh) = self.registers.get_mut("BX").unwrap(){
                                    *bl = bytes[0];
                                    *bh = bytes[1];
                                }
                            },
                        }
                    },
                    Register::CX(_, _) => {
                        match value {
                            Data::Byte(data) => {
                                if let Register::CX(cl, _) = self.registers.get_mut("CX").unwrap(){
                                    *cl = *data;
                                }
                            },
                            Data::Word(data) => {
                                let bytes = data.to_le_bytes();
                                if let Register::CX(cl, ch) = self.registers.get_mut("CX").unwrap(){
                                    *cl = bytes[0];
                                    *ch = bytes[1];
                                }
                            },
                            Data::Dword(data) => {
                                println!("Data type mismatch. Expected byte or word, found dword for instruction Mov {:?}, {}\nThis leads data truncation/loss", register, data);
                                let bytes = data.to_le_bytes();
                                if let Register::CX(cl, ch) = self.registers.get_mut("CX").unwrap(){
                                    *cl = bytes[0];
                                    *ch = bytes[1];
                                }
                            },
                        }
                    },
                    Register::DX(_, _) => {
                        match value {
                            Data::Byte(data) => {
                                if let Register::DX(dl, _) = self.registers.get_mut("DX").unwrap(){
                                    *dl = *data;
                                }
                            },
                            Data::Word(data) => {
                                let bytes = data.to_le_bytes();
                                if let Register::DX(dl, dh) = self.registers.get_mut("DX").unwrap(){
                                    *dl = bytes[0];
                                    *dh = bytes[1];
                                }
                            },
                            Data::Dword(data) => {
                                println!("Data type mismatch. Expected byte or word, found dword for instruction Mov {:?}, {}\nThis leads data truncation/loss", register, data);
                                let bytes = data.to_le_bytes();
                                if let Register::DX(dl, dh) = self.registers.get_mut("DX").unwrap(){
                                    *dl = bytes[0];
                                    *dh = bytes[1];
                                }
                            },
                        }
                    },
                    Register::EAX(_, _, _, _) => {
                        match value {
                            Data::Byte(data) => {
                                if let Register::EAX(al, _, _, _) = self.registers.get_mut("EAX").unwrap(){
                                    *al = *data;
                                }
                            },
                            Data::Word(data) => {
                                let bytes = data.to_le_bytes();
                                if let Register::EAX(al, ah, _, _) = self.registers.get_mut("EAX").unwrap(){
                                    *al = bytes[0];
                                    *ah = bytes[1];
                                }
                            },
                            Data::Dword(data) => {
                                let bytes = data.to_le_bytes();
                                if let Register::EAX(al, ah, axl, axh) = self.registers.get_mut("EAX").unwrap(){
                                    *al = bytes[0];
                                    *ah = bytes[1];
                                    *axl = bytes[2];
                                    *axh = bytes[3];
                                }
                            },
                        }
                    },
                    Register::EBX(_, _, _, _) => {
                        match value {
                            Data::Byte(data) => {
                                if let Register::EBX(bl, _, _, _) = self.registers.get_mut("EBX").unwrap(){
                                    *bl = *data;
                                }
                            },
                            Data::Word(data) => {
                                let bytes = data.to_le_bytes();
                                if let Register::EBX(bl, bh, _, _) = self.registers.get_mut("EBX").unwrap(){
                                    *bl = bytes[0];
                                    *bh = bytes[1];
                                }
                            },
                            Data::Dword(data) => {
                                let bytes = data.to_le_bytes();
                                if let Register::EBX(bl, bh, bxl, bxh) = self.registers.get_mut("EBX").unwrap(){
                                    *bl = bytes[0];
                                    *bh = bytes[1];
                                    *bxl = bytes[2];
                                    *bxh = bytes[3];
                                }
                            },
                        }
                    },
                    Register::ECX(_, _, _, _) => {
                        match value {
                            Data::Byte(data) => {
                                if let Register::ECX(cl, _, _, _) = self.registers.get_mut("ECX").unwrap(){
                                    *cl = *data;
                                }
                            },
                            Data::Word(data) => {
                                let bytes = data.to_le_bytes();
                                if let Register::ECX(cl, ch, _, _) = self.registers.get_mut("ECX").unwrap(){
                                    *cl = bytes[0];
                                    *ch = bytes[1];
                                }
                            },
                            Data::Dword(data) => {
                                let bytes = data.to_le_bytes();
                                if let Register::ECX(cl, ch, cxl, cxh) = self.registers.get_mut("ECX").unwrap(){
                                    *cl = bytes[0];
                                    *ch = bytes[1];
                                    *cxl = bytes[2];
                                    *cxh = bytes[3];
                                }
                            },
                        }
                    },
                    Register::EDX(_, _, _, _) => {
                        match value {
                            Data::Byte(data) => {
                                if let Register::EDX(dl, _, _, _) = self.registers.get_mut("EDX").unwrap(){
                                    *dl = *data;
                                }
                            },
                            Data::Word(data) => {
                                let bytes = data.to_le_bytes();
                                if let Register::EDX(dl, dh, _, _) = self.registers.get_mut("EDX").unwrap(){
                                    *dl = bytes[0];
                                    *dh = bytes[1];
                                }
                            },
                            Data::Dword(data) => {
                                let bytes = data.to_le_bytes();
                                if let Register::EDX(dl, dh, dxl, dxh) = self.registers.get_mut("EDX").unwrap(){
                                    *dl = bytes[0];
                                    *dh = bytes[1];
                                    *dxl = bytes[2];
                                    *dxh = bytes[3];
                                }
                            },
                        }
                    },
                }
            },
            (Operand::Memory(address), Operand::Register(register)) => {
                let address  = match self.memory.data_section.contains_key(address) {
                    true => {
                        address
                    },
                    false => {
                        match self.memory.bss_section.contains_key(address) {
                            true => {
                                address
                            },
                            false => {
                                println!("Use of undeclared value address {} at instruction Mov {:?}, {}", address, register, address);
                                panic!("Program terminated due to invalid memory access");
                            }
                        }
                    }
                };

                match register {
                    Register::AX(_, _) => {
                        if let Register::AX(al, ah) = self.registers.get_mut("AX").unwrap() {
                            let data = self.memory.data_section.get(address).unwrap();
                            if data.get_value() == 0 {
                                println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                                panic!("Program terminated due to invalid memory access");
                            }
                            match data {
                                Data::Byte(value) => {
                                    *al = *value;
                                },
                                Data::Word(value) => {
                                    let bytes = value.to_le_bytes();
                                    *al = bytes[0];
                                    *ah = bytes[1];
                                },
                                Data::Dword(value) => {
                                    println!("Data type mismatch. Expected byte or word, found dword at address {} for instruction Mov {:?}, {}\nThis leads data truncation/loss", address, register, address);
                                    let bytes = value.to_le_bytes();
                                    *al = bytes[0];
                                    *ah = bytes[1];
                                },
                            }

                        }
                    },
                    Register::BX(_, _) => {
                        if let Register::BX(bl, bh) = self.registers.get_mut("BX").unwrap() {
                            let data = self.memory.data_section.get(address).unwrap();
                            if data.get_value() == 0 {
                                println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                                panic!("Program terminated due to invalid memory access");
                            }
                            match data {
                                Data::Byte(value) => {
                                    *bl = *value;
                                },
                                Data::Word(value) => {
                                    let bytes = value.to_le_bytes();
                                    *bl = bytes[0];
                                    *bh = bytes[1];
                                },
                                Data::Dword(value) => {
                                    println!("Data type mismatch. Expected byte or word, found dword at address {} for instruction Mov {:?}, {}\nThis leads data truncation/loss", address, register, address);
                                    let bytes = value.to_le_bytes();
                                    *bl = bytes[0];
                                    *bh = bytes[1];
                                },
                            }

                        }
                    },
                    Register::CX(_, _) => {
                        if let Register::CX(cl, ch) = self.registers.get_mut("CX").unwrap() {
                            let data = self.memory.data_section.get(address).unwrap();
                            if data.get_value() == 0 {
                                println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                                panic!("Program terminated due to invalid memory access");
                            }
                            match data {
                                Data::Byte(value) => {
                                    *cl = *value;
                                },
                                Data::Word(value) => {
                                    let bytes = value.to_le_bytes();
                                    *cl = bytes[0];
                                    *ch = bytes[1];
                                },
                                Data::Dword(value) => {
                                    println!("Data type mismatch. Expected byte or word, found dword at address {} for instruction Mov {:?}, {}\nThis leads data truncation/loss", address, register, address);
                                    let bytes = value.to_le_bytes();
                                    *cl = bytes[0];
                                    *ch = bytes[1];
                                },
                            }
                        }
                    },
                    Register::DX(_, _) => {
                        if let Register::DX(dl, dh) = self.registers.get_mut("DX").unwrap() {
                            let data = self.memory.data_section.get(address).unwrap();
                            if data.get_value() == 0 {
                                println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                                panic!("Program terminated due to invalid memory access");
                            }
                            match data {
                                Data::Byte(value) => {
                                    *dl = *value;
                                },
                                Data::Word(value) => {
                                    let bytes = value.to_le_bytes();
                                    *dl = bytes[0];
                                    *dh = bytes[1];
                                },
                                Data::Dword(value) => {
                                    println!("Data type mismatch. Expected byte or word, found dword at address {} for instruction Mov {:?}, {}\nThis leads data truncation/loss", address, register, address);
                                    let bytes = value.to_le_bytes();
                                    *dl = bytes[0];
                                    *dh = bytes[1];
                                },
                            }
                        }
                    },
                    Register::EAX(_, _, _, _) => {
                        if let Register::EAX(al, ah, axl, axh) = self.registers.get_mut("EAX").unwrap() {
                            let data = self.memory.data_section.get(address).unwrap();
                            if data.get_value() == 0 {
                                println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                                panic!("Program terminated due to invalid memory access");
                            }
                            match data {
                                Data::Byte(value) => {
                                    *al = *value;
                                },
                                Data::Word(value) => {
                                    let bytes = value.to_le_bytes();
                                    *al = bytes[0];
                                    *ah = bytes[1];
                                },
                                Data::Dword(value) => {
                                    let bytes = value.to_le_bytes();
                                    *al = bytes[0];
                                    *ah = bytes[1];
                                    *axl = bytes[2];
                                    *axh = bytes[3];
                                },
                            }
                        }
                    },
                    Register::EBX(_, _, _, _) => {
                        if let Register::EBX(bl, bh, bxl, bxh) = self.registers.get_mut("EBX").unwrap() {
                            let data = self.memory.data_section.get(address).unwrap();
                            if data.get_value() == 0 {
                                println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                                panic!("Program terminated due to invalid memory access");
                            }
                            match data {
                                Data::Byte(value) => {
                                    *bl = *value;
                                },
                                Data::Word(value) => {
                                    let bytes = value.to_le_bytes();
                                    *bl = bytes[0];
                                    *bh = bytes[1];
                                },
                                Data::Dword(value) => {
                                    let bytes = value.to_le_bytes();
                                    *bl = bytes[0];
                                    *bh = bytes[1];
                                    *bxl = bytes[2];
                                    *bxh = bytes[3];
                                },
                            }
                        }
                    },
                    Register::ECX(_, _, _, _) => {
                        if let Register::ECX(cl, ch, cxl, cxh) = self.registers.get_mut("ECX").unwrap() {
                            let data = self.memory.data_section.get(address).unwrap();
                            if data.get_value() == 0 {
                                println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                                panic!("Program terminated due to invalid memory access");
                            }
                            match data {
                                Data::Byte(value) => {
                                    *cl = *value;
                                },
                                Data::Word(value) => {
                                    let bytes = value.to_le_bytes();
                                    *cl = bytes[0];
                                    *ch = bytes[1];
                                },
                                Data::Dword(value) => {
                                    let bytes = value.to_le_bytes();
                                    *cl = bytes[0];
                                    *ch = bytes[1];
                                    *cxl = bytes[2];
                                    *cxh = bytes[3];
                                },
                            }
                        }
                    },
                    Register::EDX(_, _, _, _) => {
                        if let Register::EDX(dl, dh, dxl, dxh) = self.registers.get_mut("EDX").unwrap() {
                            let data = self.memory.data_section.get(address).unwrap();
                            if data.get_value() == 0 {
                                println!("Use of uninitialized value address {} at instruction Mov {:?}, {}", address, register, address);
                                panic!("Program terminated due to invalid memory access");
                            }
                            match data {
                                Data::Byte(value) => {
                                    *dl = *value;
                                },
                                Data::Word(value) => {
                                    let bytes = value.to_le_bytes();
                                    *dl = bytes[0];
                                    *dh = bytes[1];
                                },
                                Data::Dword(value) => {
                                    let bytes = value.to_le_bytes();
                                    *dl = bytes[0];
                                    *dh = bytes[1];
                                    *dxl = bytes[2];
                                    *dxh = bytes[3];
                                },
                            }
                        }
                    },
                }
            },
            (Operand::Memory(address), Operand::Immediate(value)) => {
                let address = match self.memory.data_section.contains_key(address) {
                    true => {
                        address
                    },
                    false => {
                        match self.memory.bss_section.contains_key(address) {
                            true => {
                                address
                            },
                            false => {
                                println!("Use of undeclared value address {0} at instruction Mov {0}, {1:?}", address, value);
                                panic!("Program terminated due to invalid memory access");
                            }
                        }
                    }
                };

                match value {
                    Data::Byte(data) => {
                        if let Some(address_value) = self.memory.data_section.get_mut(address){
                            match address_value {
                                Data::Byte(address_data) => {
                                    *address_data = *data;
                                },
                                Data::Word(address_data) => {
                                    println!("Data type mismatch. Expected byte, found word at address {} for instruction Mov {}, {}\nThis leads memory wastage", address, address, data);
                                    *address_data = *data as u16;
                                },
                                Data::Dword(address_data) => {
                                    println!("Data type mismatch. Expected byte, found dword at address {} for instruction Mov {}, {}\nThis leads memory wastage", address, address, data);
                                    *address_data = *data as u32;
                                }
                            }
                        }
                    },
                    Data::Word(data) => {
                        if let Some(address_value) = self.memory.data_section.get_mut(address){
                            match address_value {
                                Data::Byte(address_data) => {
                                    println!("Data type mismatch. Expected word, found byte at address {} for instruction Mov {}, {}\nThis leads data truncation/loss", address, address, data);
                                    *address_data = (data & 0x00FF) as u8;
                                },
                                Data::Word(address_data) => {
                                    *address_data = *data;
                                },
                                Data::Dword(address_data) => {
                                    println!("Data type mismatch. Expected word, found dword at address {} for instruction Mov {}, {}\nThis leads memory wastage", address, address, data);
                                    *address_data = *data as u32;
                                }
                            }
                        }
                    },
                    Data::Dword(data) => {
                        if let Some(address_value) = self.memory.data_section.get_mut(address){
                            match address_value {
                                Data::Byte(address_data) => {
                                    println!("Data type mismatch. Expected dword, found byte at address {} for instruction Mov {}, {}\nThis leads data truncation/loss", address, address, data);
                                    *address_data = (data & 0x000000FF) as u8;
                                },
                                Data::Word(address_data) => {
                                    println!("Data type mismatch. Expected dword, found word at address {} for instruction Mov {}, {}\nThis leads data truncation/loss", address, address, data);
                                    *address_data = (data & 0x0000FFFF) as u16;
                                },
                                Data::Dword(address_data) => {
                                    *address_data = *data;
                                }
                            }
                        }
                    },
                }
            },
            _ => {
                println!("Invalid addressing mode for move instruction Mov {:?},{:?}\nEnsure:\n1. Immediate value isn't used as destination\n2. No memory to memory moves", destination, source);
                return;
            }
        }
        self.display_registers();
    }

    fn execute(&mut self){
        let instruction = match self.fetch() {
            Some(i) => i,
            None => {
                println!("program terminated successfully");
                return
            }
        };

        match instruction.opcode {
            IS::Mov => {
                let dest = &instruction.operands.get(0);
                let src = &instruction.operands.get(1);

                match (dest, src) {
                    (Some(destination), Some(source)) => {
                        self.mov(*source, *destination);
                    },
                    _ => {
                        println!("Insufficient operands for move instruction {:?}\nExpected Mov destination, source", instruction);
                        return;
                    }
                }

            },
            IS::Add => {
                let dest = &instruction.operands[0];
                let src = &instruction.operands[1];
                match (dest, src) {
                    (Operand::Register(dest_reg), Operand::Register(src_reg)) => {

                    },

                    (Operand::Register(register), Operand::Memory(_)) => {

                    },

                    (Operand::Register(register), Operand::Immediate(_)) => {

                    },

                    (Operand::Memory(_), Operand::Register(register)) => {

                    },
                    (Operand::Memory(_), Operand::Memory(_)) => {

                    },
                    (Operand::Memory(_), Operand::Immediate(_)) => {

                    },

                    (Operand::Immediate(_), Operand::Register(register)) => {
                        println!("Immediate to register addition not allowed")
                    },

                    (Operand::Immediate(_), Operand::Memory(_)) => {
                        println!("Immediate to memory addition not allowed")
                    },

                    (Operand::Immediate(_), Operand::Immediate(_)) => {
                        println!("Immediate to immediate addition not allowed")
                    },
                }
            },
            IS::Sub => {
                let dest = &instruction.operands[0];
                let src = &instruction.operands[1];
                match (dest, src) {
                    (Operand::Register(dest_reg), Operand::Register(src_reg)) => {

                    },

                    (Operand::Register(register), Operand::Memory(_)) => {

                    },

                    (Operand::Register(register), Operand::Immediate(_)) => {

                    },

                    (Operand::Memory(_), Operand::Register(register)) => {

                    },
                    (Operand::Memory(_), Operand::Memory(_)) => {

                    },
                    (Operand::Memory(_), Operand::Immediate(_)) => {

                    },

                    (Operand::Immediate(_), Operand::Register(register)) => {
                        println!("Immediate to register subtraction not allowed")
                    },

                    (Operand::Immediate(_), Operand::Memory(_)) => {
                        println!("Immediate to memory subtraction not allowed")
                    },

                    (Operand::Immediate(_), Operand::Immediate(_)) => {
                        println!("Immediate to immediate subtraction not allowed")
                    },
                }
            },
            // IS::Mul => {
            //     let dest = &instruction.operands[0];
            //     let src = &instruction.operands[1];
            //     self.mul(dest, src);
            // },
            // IS::Div => {
            //     let dest = &instruction.operands[0];
            //     let src = &instruction.operands[1];
            //     self.div(dest, src);
            // },
            // IS::And => {
            //     let dest = &instruction.operands[0];
            //     let src = &instruction.operands[1];
            //     self.and(dest, src);
            // },
            // IS::Or => {
            //     let dest = &instruction.operands[0];
            //     let src = &instruction.operands[1];
            //     self.or(dest, src);
            // },
            // IS::Xor => {
            //     let dest = &instruction.operands[0];
            //     let src = &instruction.operands[1];
            //     self.xor(dest, src);
            // },
            // IS::Not => {
            //     let dest = &instruction.operands[0];
            //     self.not(dest);
            // },
            // IS::Cmp => {
            //     let dest = &instruction.operands[0];
            //     let src = &instruction.operands[1];
            //     self.cmp(dest, src);
            // },
            // IS::Jmp => {
            //     let dest = &instruction.operands[0];
            //     self.jmp(dest);
            // },
            // IS::Je => {
            //     let dest = &instruction.operands[0];
            //     self.je(dest);
            // },
            // IS::Jne => {
            //     let dest = &instruction.operands[0];
            //     self.jne(dest);
            // },
            // IS::Jg => {
            //     let dest = &instruction.operands[0];
            //     self.jg(dest);
            // },
            // IS::Jge => {
            //     let dest = &instruction.operands[0];
            //     self.jge(dest);
            // },
            _ => {
                println!("Instruction {:?} not implemented yet", instruction.opcode)

            }
        }
        self.display_registers();
    }
}

fn main() {

    let data_section: HashMap<String, Data> = HashMap::from([
        ("num1".to_string(), Data::Byte(23)),
        ("num2".to_string(), Data::Word(300)),
    ]);

    let bss_section: HashMap<String, Data> = HashMap::from([
        ("result".to_string(), Data::Word(0)),
        ("test".to_string(), Data::Dword(0)),
    ]);

    let text_section: Vec<Instruction> = vec![
        Instruction{
            opcode: IS::Mov,
            operands: vec![
                Operand::Register(Register::AX(0, 0)),
                Operand::Immediate(Data::Byte(23)),
            ],
        },
        Instruction{
            opcode: IS::Mov,
            operands: vec![
                Operand::Register(Register::BX(0, 0)),
                Operand::Register(Register::AX(0, 0)),
            ],
        },
        Instruction{
            opcode: IS::Mov,
            operands: vec![
                Operand::Memory("num1".to_string()),
                Operand::Immediate(Data::Dword(0)),
            ],
        },
        Instruction{
            opcode: IS::Mov,
            operands: vec![
                Operand::Memory("num2".to_string()),
                Operand::Immediate(Data::Byte(0)),
            ],
        },
        Instruction{
            opcode: IS::Mov,
            operands: vec![
                Operand::Register(Register::EAX(0, 0, 0, 0)),
                Operand::Register(Register::BX(0, 0)),
            ],
        },
        Instruction{
            opcode: IS::Mov,
            operands: vec![
                Operand::Memory("result".to_string()),
                Operand::Register(Register::EAX(0, 0, 0, 0)),
            ],
        },
    ];

    let program = Program{
        data_section,
        bss_section,
        text_section,
    };

    let mut cpu = CPU::new(program);

    cpu.display_registers();
    cpu.run();
}
