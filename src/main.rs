use std::{collections::HashMap, fmt::Debug};



trait GetValue<T> {
    fn get_value(&self) -> T;
}

trait SetValue<T> {
    fn set_value(&mut self, value: T);
}

trait DisplayRegister: std::fmt::Debug {
    fn display(&self){
        println!("{:?}", self);   
    }
}

#[derive(Debug)]
/// Registers type used to store different register types of the CPU
struct Registers{
    GP: [GPRegister; 8],
    SP: [SPRegister; 3],
    FLAGS: [FLAGS; 9],
}

impl DisplayRegister for Registers {
    fn display(&self) {
        println!("General Purpose Registers:");
        self.GP.iter().for_each(|reg| {
            println!("{:?}", reg);
        });

        println!("Special Purpose Registers:");
        self.SP.iter().for_each(|reg| {
            println!("{:?}", reg);
        });

        println!("Flags:");
        self.FLAGS.iter().for_each(|flag| {
            println!("{:?}", flag);
        });
    }
}

impl Registers {
    fn get_register(&mut self, register: Register) -> &mut GPRegister {
        match register {
            Register::AX => &mut self.GP[0],
            Register::BX => &mut self.GP[1],
            Register::CX => &mut self.GP[2],
            Register::DX => &mut self.GP[3],
            Register::EAX => &mut self.GP[4],
            Register::EBX => &mut self.GP[5],
            Register::ECX => &mut self.GP[6],
            Register::EDX => &mut self.GP[7],
        }
    }
}

#[derive(Debug, Clone)]
/// General Purpose Registers for user interfacing(usage) when writing Instructions
enum Register{
    AX, BX, CX, DX,
    EAX, EBX, ECX, EDX,
}

#[derive(Clone, Debug)]
///General Purpose Registers
enum GPRegister {
    AX(u8, u8),
    BX(u8, u8),
    CX(u8, u8),
    DX(u8, u8),
    EAX(u8, u8, u8, u8),
    EBX(u8, u8, u8, u8),
    ECX(u8, u8, u8, u8),
    EDX(u8, u8, u8, u8),
}



impl GetValue<u32> for GPRegister {
    fn get_value(&self) -> u32 {
        match self {
            GPRegister::AX(a, b) => u16::from_le_bytes([*a, *b]) as u32,
            GPRegister::BX(a, b) => u16::from_le_bytes([*a, *b]) as u32,
            GPRegister::CX(a, b) => u16::from_le_bytes([*a, *b]) as u32,
            GPRegister::DX(a, b) => u16::from_le_bytes([*a, *b]) as u32,
            GPRegister::EAX(a, b, c, d) => u32::from_le_bytes([*a, *b, *c, *d]),
            GPRegister::EBX(a, b, c, d) => u32::from_le_bytes([*a, *b, *c, *d]),
            GPRegister::ECX(a, b, c, d) => u32::from_le_bytes([*a, *b, *c, *d]),
            GPRegister::EDX(a, b, c, d) => u32::from_le_bytes([*a, *b, *c, *d]),
        }
    }
}

//TODO: Implementing the SetValue trait for the GPRegister enum
//TODO: Ensure it stores values as bytes in Little Endian format for the corresponding register
//TODO: Empty the source register after moving the value to the destination register.
impl SetValue<Data> for GPRegister {
    fn set_value(&mut self, value: Data) {
        match self {
            GPRegister::AX(_, ah) => {
                match value {
                    Data::Byte(value) => *self = GPRegister::AX(value, *ah),
                    Data::Word(value) => {
                        let data = value.to_le_bytes();
                        *self = GPRegister::AX(data[0], data[1]);
                    }
                    _ => {
                        panic!("Data type mismatch. Expected Word or Byte, found Dword");
                    }
                }
            },

            GPRegister::BX(_, bh) => {
                match value {
                    Data::Byte(value) => *self = GPRegister::BX(value, *bh),
                    Data::Word(value) => {
                        let data = value.to_le_bytes();
                        *self = GPRegister::BX(data[0], data[1]);
                    }
                    _ => {
                        panic!("Data type mismatch. Expected Word or Byte, found Dword");
                    }
                }
            },

            GPRegister::CX(_, ch) => {
                match value {
                    Data::Byte(value) => *self = GPRegister::CX(value, *ch),
                    Data::Word(value) => {
                        let data = value.to_le_bytes();
                        *self = GPRegister::CX(data[0], data[1]);
                    }
                    _ => {
                        panic!("Data type mismatch. Expected Word or Byte, found Dword");
                    }
                }
            },

            GPRegister::DX(_, dh) => {
                match value {
                    Data::Byte(value) => *self = GPRegister::DX(value, *dh),
                    Data::Word(value) => {
                        let data = value.to_le_bytes();
                        *self = GPRegister::DX(data[0], data[1]);
                    }
                    _ => {
                        panic!("Data type mismatch. Expected Word or Byte, found Dword");
                    }
                }
            },

            GPRegister::EAX(_, ah, eal, eah) => {
                match value {
                    Data::Byte(a) => {
                        *self = GPRegister::EAX(a, *ah, *eal, *eah);
                    }
                    Data::Word(a) => {
                        let ah = (a >> 8) as u8;
                        let al = (a & 0x00FF) as u8;
                        *self = GPRegister::EAX(al, ah, *eal, *eah);
                    }
                    Data::Dword(a) => {
                        let eah = (a >> 16) as u8;
                        let eal = (a >> 8) as u8;
                        let ah = (a >> 24) as u8;
                        let al = (a & 0x00FF) as u8;
                        *self = GPRegister::EAX(al, ah, eal, eah);
                    }
                }
            },

            GPRegister::EBX(_, bh, ebl, ebh) => {
                match value {
                    Data::Byte(a) => {
                        *self = GPRegister::EBX(a, *bh, *ebl, *ebh);
                    }
                    Data::Word(a) => {
                        let bh = (a >> 8) as u8;
                        let bl = (a & 0x00FF) as u8;
                        *self = GPRegister::EBX(bl, bh, *ebl, *ebh);
                    }
                    Data::Dword(a) => {
                        let ebh = (a >> 16) as u8;
                        let ebl = (a >> 8) as u8;
                        let bh = (a >> 24) as u8;
                        let bl = (a & 0x00FF) as u8;
                        *self = GPRegister::EBX(bl, bh, ebl, ebh);
                    }
                }
            },

            GPRegister::ECX(_, ch, ecl, ech) => {
                match value {
                    Data::Byte(a) => {
                        *self = GPRegister::ECX(a, *ch, *ecl, *ech);
                    }
                    Data::Word(a) => {
                        let ch = (a >> 8) as u8;
                        let cl = (a & 0x00FF) as u8;
                        *self = GPRegister::ECX(cl, ch, *ecl, *ech);
                    }
                    Data::Dword(a) => {
                        let ech = (a >> 16) as u8;
                        let ecl = (a >> 8) as u8;
                        let ch = (a >> 24) as u8;
                        let cl = (a & 0x00FF) as u8;
                        *self = GPRegister::ECX(cl, ch, ecl, ech);
                    }
                }
            },

            GPRegister::EDX(_, dh, edl, edh) => {
                match value {
                    Data::Byte(a) => {
                        *self = GPRegister::EDX(a, *dh, *edl, *edh);
                    }
                    Data::Word(a) => {
                        let dh = (a >> 8) as u8;
                        let dl = (a & 0x00FF) as u8;
                        *self = GPRegister::EDX(dl, dh, *edl, *edh);
                    }
                    Data::Dword(a) => {
                        let edh = (a >> 16) as u8;
                        let edl = (a >> 8) as u8;
                        let dh = (a >> 24) as u8;
                        let dl = (a & 0x00FF) as u8;
                        *self = GPRegister::EDX(dl, dh, edl, edh);
                    }
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
///Special Purpose Registers
enum SPRegister {
    SP(u8, u8),
    BP(u8, u8),
    IP(u8, u8),
}

impl GetValue<u32>for SPRegister {
    fn get_value(&self) -> u32 {
        match self {
            SPRegister::SP(a, b) => u16::from_le_bytes([*a, *b]) as u32,
            SPRegister::BP(a, b) => u16::from_le_bytes([*a, *b]) as u32,
            SPRegister::IP(a, b) => u16::from_le_bytes([*a, *b]) as u32,
        }
    }
}

impl SetValue<Data> for SPRegister {
    fn set_value(&mut self, value: Data) {
        match self {
            SPRegister::SP(_, b) => {
                match value {
                    Data::Byte(a) => {
                        *self = SPRegister::SP(a, *b);
                    }
                    Data::Word(a) => {
                        let b = (a >> 8) as u8;
                        let a = (a & 0x00FF) as u8;
                        *self = SPRegister::SP(a, b);
                    }
                    Data::Dword(a) => {
                        let b = (a >> 16) as u8;
                        let a = (a & 0x00FF) as u8;
                        *self = SPRegister::SP(a, b);
                    }
                }
            },

            SPRegister::BP(_, b) => {
                match value {
                    Data::Byte(a) => {
                        *self = SPRegister::BP(a, *b);
                    }
                    Data::Word(a) => {
                        let b = (a >> 8) as u8;
                        let a = (a & 0x00FF) as u8;
                        *self = SPRegister::BP(a, b);
                    }
                    Data::Dword(a) => {
                        let b = (a >> 16) as u8;
                        let a = (a & 0x00FF) as u8;
                        *self = SPRegister::BP(a, b);
                    }
                }
            },

            SPRegister::IP(_, b) => {
                match value {
                    Data::Byte(a) => {
                        *self = SPRegister::IP(a, *b);
                    }
                    Data::Word(a) => {
                        let b = (a >> 8) as u8;
                        let a = (a & 0x00FF) as u8;
                        *self = SPRegister::IP(a, b);
                    }
                    Data::Dword(a) => {
                        let b = (a >> 16) as u8;
                        let a = (a & 0x00FF) as u8;
                        *self = SPRegister::IP(a, b);
                    }
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
enum FLAGS {
    PF(u8), AF(u8), ZF(u8),
    SF(u8), TF(u8), IF(u8),
    DF(u8), OF(u8), CF(u8),
}

impl GetValue<u8> for FLAGS {
    fn get_value(&self) -> u8 {
        match self {
            FLAGS::AF(a) => *a, FLAGS::ZF(a) => *a,
            FLAGS::SF(a) => *a, FLAGS::TF(a) => *a,
            FLAGS::IF(a) => *a, FLAGS::DF(a) => *a,
            FLAGS::OF(a) => *a, FLAGS::CF(a) => *a,
            FLAGS::PF(a) => *a,
        }
    }
}

impl SetValue<u8> for FLAGS {
    fn set_value(&mut self, value: u8) {
        match self {
            FLAGS::ZF(a) => *a = value, FLAGS::SF(a) => *a = value,
            FLAGS::TF(a) => *a = value, FLAGS::IF(a) => *a = value,
            FLAGS::DF(a) => *a = value, FLAGS::OF(a) => *a = value,
            FLAGS::CF(a) => *a = value, FLAGS::PF(a) => *a = value,
            FLAGS::AF(a) => *a = value,
        }
    }
}

#[derive(Debug, Clone)]
///! Instruction Set. This is the set of instructions that the CPU can execute.
/// NB: Not all instructions are implemented.
enum IS {
    Mov, Add, Sub,
    Mul, Div, And,
    Or, Xor, Not
}

#[derive(Debug, Clone)]
/// Data type used to store data in memory
/// NB: Only Byte, Word and Dword are supported
enum Data {
    Byte(u8),
    Word(u16),
    Dword(u32),
}

impl GetValue<u32> for Data {
    fn get_value(&self) -> u32 {
        match self {
            Data::Byte(a) => *a as u32,
            Data::Word(a) => *a as u32,
            Data::Dword(a) => *a,
        }
    }
    
}

#[derive(Debug, Clone)]
/// Operand type used to store operands for instructions
/// Usage example:
/// ```
/// Instruction::new(
///     IS::Mov, vec![Operand::Register(Register::AX), Operand::Immediate(Data::Word(0x00FF))]
/// );
/// ```
/// This example moves the value 0x00FF to the AX register
/// It simulates the instruction `MOV AX, 0x00FF` in x86 assembly
/// ```
enum Operand {
    Register(Register),
    Memory(String),
    Immediate(Data),
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: IS,
    operands: Vec<Operand>,
    operand_count: u8,
}

impl Instruction {
    fn new(opcode: IS, operands: Vec<Operand>) -> Instruction {
        Instruction {
            operand_count: operands.len() as u8,
            opcode,
            operands,
        }
    }

    fn verify_operands(&self) -> bool {
        match self.opcode {
            IS::Mov => {
                match self.operand_count {
                    2 => true,
                    _ => false
                }
            },
            IS::Add => {
                match self.operand_count {
                    2 => true,
                    _ => false,
                }
            },
            IS::Sub => {
                match self.operand_count {
                    2 => true,
                    _ => false,
                }
            },
            _ => panic!("Unsupported Instruction"),
            
        }
    }
}

#[derive(Debug)]
/// Memory Unit
/// This is the unit that stores data and code sections
/// It is used to simulate the memory of the CPU
struct MemoryUnit {
    data_section: HashMap<String, Data>,
    code_section: Vec<Instruction>,
}

#[derive(Debug)]
enum ALUMode {
    Add, Sub, Mul,
    Div, And, Or,
    Xor, Not, Off
}

#[derive(Debug)]
/// Arithmetic Logic Unit
/// This is the unit that performs arithmetic and logical operations
/// All operations assume u8 values
struct ALU{
    buffers: Vec<u8>,
    mode: ALUMode,
}

impl ALU {
    fn new() -> ALU {
        ALU {
            buffers: Vec::new(),
            mode: ALUMode::Off,
        }
    }

    /// Sets the mode of the ALU's operation state
    fn set_mode(&mut self, mode: ALUMode) {
        self.mode = mode;
    }

    /// Executes the operation based on the mode of the ALU
    fn execute(&mut self, a: u8, b: u8) -> (u8, bool) {
        match self.mode {
            ALUMode::Add => self.add(a, b),
            ALUMode::Sub => self.sub(a, b),
            ALUMode::Off => panic!("ALU is off"),
            _ => panic!("Unsupported mode not implemented"),
        }
    }

    /// Adds two u8 values and returns the result and a boolean indicating if there was an overflow
    fn add(&mut self, a: u8, b: u8) -> (u8, bool) {
        a.overflowing_add(b)
    } 

    /// Subtracts two u8 values and returns the result and a boolean indicating if there was an overflow
    fn sub(&mut self, a: u8, b: u8) -> (u8, bool) {
        a.overflowing_sub(b)
    }
}

#[derive(Debug)]
/// Central Processing Unit
/// This is the main unit that controls the execution of the program
/// It contains the ALU, Registers and Memory Unit
struct CPU {
    alu: ALU,
    registers: Registers,
    memory_unit: MemoryUnit,
}

impl CPU {
    fn new(data_section: HashMap<String, Data>, code_section: Vec<Instruction>)-> CPU {
        CPU {
            alu: ALU::new(),
            registers: Registers {
                GP: [GPRegister::AX(0, 0), GPRegister::BX(0, 0), GPRegister::CX(0, 0), GPRegister::DX(0, 0), GPRegister::EAX(0, 0, 0, 0), GPRegister::EBX(0, 0, 0, 0), GPRegister::ECX(0, 0, 0, 0), GPRegister::EDX(0, 0, 0, 0)],
                SP: [SPRegister::SP(0, 0), SPRegister::BP(0, 0), SPRegister::IP(0, 0)],
                FLAGS: [FLAGS::PF(0), FLAGS::AF(0), FLAGS::ZF(0), FLAGS::SF(0), FLAGS::TF(0), FLAGS::IF(0), FLAGS::DF(0), FLAGS::OF(0), FLAGS::CF(0)],
            },
            memory_unit: MemoryUnit {
                data_section,
                code_section,
            },
        }
    }

    fn run(&mut self){
        if self.memory_unit.code_section.len() == 0 {
            println!("Program is empty");
            return;
        }
        loop {
            self.fetch();
            if self.registers.SP[2].get_value() >= self.memory_unit.code_section.len() as u32 {
                break;
            }
        }
    }

    /// The fetch stage operation of CPU's workflow
    fn fetch(&mut self) {
            let pc = self.registers.SP[2].get_value();
            let instruction = self.memory_unit.code_section[pc as usize].clone();
            self.registers.SP[2].set_value(Data::Word((pc + 1) as u16));
            self.decode(instruction);
        }

    /// The decode stage operation of CPU's workflow
    fn decode(&mut self, instruction: Instruction) {
        match instruction.opcode {
            IS::Mov => {
                match instruction.verify_operands() {
                    false => {
                        panic!("Invalid operands for MOV instruction at {0:?} Mov expects only 2 operands", instruction);
                    },
                    _ => {}
                }

                let dest = instruction.operands[0].clone();
                let src = instruction.operands[1].clone();
                match (dest, src) {
                    (Operand::Register(dest_register), Operand::Register(src_register)) => {
                        let src_value = self.registers.get_register(src_register).get_value();
                        let mut dest_reg = self.registers.get_register(dest_register);
                        match dest_reg {
                            GPRegister::AX(_, _) => dest_reg.set_value(Data::Word(src_value as u16)),
                            GPRegister::BX(_, _) => dest_reg.set_value(Data::Word(src_value as u16)),
                            GPRegister::CX(_, _) => dest_reg.set_value(Data::Word(src_value as u16)),
                            GPRegister::DX(_, _) => dest_reg.set_value(Data::Word(src_value as u16)),
                            GPRegister::EAX(_, _, _, _) => dest_reg.set_value(Data::Dword(src_value)),
                            GPRegister::EBX(_, _, _, _) => dest_reg.set_value(Data::Dword(src_value)),
                            GPRegister::ECX(_, _, _, _) => dest_reg.set_value(Data::Dword(src_value)),
                            GPRegister::EDX(_, _, _, _) => dest_reg.set_value(Data::Dword(src_value)),
                        }
                    },
                    (Operand::Register(register), Operand::Memory(address)) => {
                        let src_value = self.memory_unit.data_section[&address].get_value();
                        let mut dest_reg = self.registers.get_register(register);
                        match dest_reg {
                            GPRegister::AX(_, _) => dest_reg.set_value(Data::Word(src_value as u16)),
                            GPRegister::BX(_, _) => dest_reg.set_value(Data::Word(src_value as u16)),
                            GPRegister::CX(_, _) => dest_reg.set_value(Data::Word(src_value as u16)),
                            GPRegister::DX(_, _) => dest_reg.set_value(Data::Word(src_value as u16)),
                            GPRegister::EAX(_, _, _, _) => dest_reg.set_value(Data::Dword(src_value)),
                            GPRegister::EBX(_, _, _, _) => dest_reg.set_value(Data::Dword(src_value)),
                            GPRegister::ECX(_, _, _, _) => dest_reg.set_value(Data::Dword(src_value)),
                            GPRegister::EDX(_, _, _, _) => dest_reg.set_value(Data::Dword(src_value)),
                        }
                    },
                    (Operand::Register(register), Operand::Immediate(value)) => {
                        let data = value.get_value();
                        let dest_reg = self.registers.get_register(register);
                        match dest_reg {
                            GPRegister::AX(_, _) => dest_reg.set_value(Data::Word(data as u16)),
                            GPRegister::BX(_, _) => dest_reg.set_value(Data::Word(data as u16)),
                            GPRegister::CX(_, _) => dest_reg.set_value(Data::Word(data as u16)),
                            GPRegister::DX(_, _) => dest_reg.set_value(Data::Word(data as u16)),
                            GPRegister::EAX(_, _, _, _) => dest_reg.set_value(Data::Dword(data)),
                            GPRegister::EBX(_, _, _, _) => dest_reg.set_value(Data::Dword(data)),
                            GPRegister::ECX(_, _, _, _) => dest_reg.set_value(Data::Dword(data)),
                            GPRegister::EDX(_, _, _, _) => dest_reg.set_value(Data::Dword(data)),
                        }
                    },
                    (Operand::Memory(address), Operand::Register(register)) => {
                        let src_value = self.registers.get_register(register.clone()).get_value();
                        if let None = self.memory_unit.data_section.get_mut(&address) {
                           println!("Use of undeclared memory address: {:?}", address);
                           panic!("Invalid memory address at {:?}", instruction);
                        }

                        let data = match self.registers.get_register(register) {
                            GPRegister::AX(_, _) => Data::Word(src_value as u16),
                            GPRegister::BX(_, _) => Data::Word(src_value as u16),
                            GPRegister::CX(_, _) => Data::Word(src_value as u16),
                            GPRegister::DX(_, _) => Data::Word(src_value as u16),
                            GPRegister::EAX(_, _, _, _) => Data::Dword(src_value),
                            GPRegister::EBX(_, _, _, _) => Data::Dword(src_value),
                            GPRegister::ECX(_, _, _, _) => Data::Dword(src_value),
                            GPRegister::EDX(_, _, _, _) => Data::Dword(src_value),
                        };

                        self.memory_unit.data_section.insert(address, data);
                    },
                    (Operand::Memory(address), Operand::Immediate(value)) => {
                        if let None = self.memory_unit.data_section.get_mut(&address) {
                            println!("Use of undeclared memory address: {:?}", address);
                            panic!("Invalid memory address at {:?}", instruction);
                        }

                        self.memory_unit.data_section.insert(address, value);
                    },
                    _ => {
                        panic!("Invalid operands for MOV instruction at {0:?} Be sure that:\n1. Immediate value isn't used as destination.\n2. Movement from memory to memory aren't possible{0:?}", instruction);
                    }
                }
            },
            IS::Add => {},
            IS::Sub => {},
            _ => panic!("Unsupported Instruction at {:?}", instruction),
        }
        self.display_registers();
    }

    fn display_registers(&self) {
        self.registers.GP.iter().for_each(|reg| {
            println!("{:?}", reg);
        });
    }
}

fn main(){
    let data_section: HashMap<String, Data> = HashMap::from([
        ("num".to_string(), Data::Word(10)),
        ("num2".to_string(), Data::Word(20)),
        ("result".to_string(), Data::Word(0)),
    ]);

    let code_section: Vec<Instruction> = vec![
        Instruction::new(IS::Mov, vec![Operand::Register(Register::AX), Operand::Immediate(Data::Word(24))]),
        Instruction::new(IS::Mov, vec![Operand::Register(Register::BX), Operand::Memory("num".to_string())]),
        Instruction::new(IS::Mov, vec![Operand::Register(Register::CX), Operand::Register(Register::AX)]),
        Instruction::new(IS::Mov, vec![Operand::Memory("result".to_string()), Operand::Register(Register::BX)]),
        Instruction::new(IS::Mov, vec![Operand::Memory("num2".to_string()), Operand::Immediate(Data::Word(0x00FF))]),
    ];
    let mut cpu = CPU::new(data_section, code_section);
    cpu.run();
}