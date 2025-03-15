/// This is a simple implementation of a CPU in Rust
/// It is meant to assume simple assembly instructions, say, mov, add, sub, etc.
/// Example intended usage;
/// ```
/// section .data
/// num1 dw 10       ; First number (16-bit integer)
/// num2 dw 20       ; Second number (16-bit integer)
/// result dd 0      ; Variable to store the result
/// newline db 10    ; Newline character for formatting

/// section .bss
/// output resb 10   ; Buffer to store the result as a string

/// section .text
/// global _start

/// _start:
// / ; Load values from memory
/// mov eax, [num1]  ; Load first number into EAX
/// add eax, [num2]  ; Add second number

/// ; Store the result in memory
/// mov [result], eax

/// ; Convert result to string (integer to ASCII)
/// mov edi, output  ; Destination buffer
/// call int_to_str  ; Convert EAX to ASCII string

/// ; Print the result
/// mov eax, 1       ; syscall: sys_write
/// mov edi, 1       ; file descriptor: stdout
/// mov rsi, output  ; buffer
/// mov rdx, 10      ; max 10 bytes
/// syscall

/// ; Print newline
/// mov eax, 1
/// mov edi, 1
/// mov rsi, newline
/// mov rdx, 1
/// syscall

/// ; Exit program
/// mov eax, 60      ; syscall: sys_exit
/// xor edi, edi     ; status 0
/// syscall

/// ; -----------------------------------
/// ; Convert integer in EAX to ASCII
/// ; -----------------------------------
/// int_to_str:
/// mov ecx, 10      ; Base 10 divisor
/// mov ebx, edi     ; Save buffer pointer

/// .loop:
///     xor edx, edx
///     div ecx      ; EAX = EAX / 10, remainder in EDX
///     add dl, '0'  ; Convert to ASCII
///     dec edi
///     mov [edi], dl
///     test eax, eax
///     jnz .loop

/// mov rsi, edi     ; Update buffer pointer
/// ret
///```
/// The above code is a simple assembly code that adds two numbers and prints the result

use std::{collections::HashMap, fmt::Debug, io::{stdin, Read, stdout, Write}};


trait GetValue<T> {
    fn get_value(&self) -> T;
}

trait SetValue<T, U> {
    fn set_value(&mut self, value: T) -> U;
}

trait DisplayRegister: std::fmt::Debug {
    fn display(&self){
        println!("{:?}", self);   
    }
}

#[derive(Debug, Clone)]
/// General Purpose Registers for user interfacing(usage) when writing Instructions
enum Register{
    AX, BX, CX, DX,
    EAX, EBX, ECX, EDX,
}

#[allow(non_snake_case)]
#[derive(Debug)]
/// Registers type used to store different register types of the CPU
struct Registers{
    GP: [GPRegister; 8],
    SP: [SPRegister; 3],
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
    }
}

impl Registers {
    fn get_register(&mut self, register: Register) -> &mut GPRegister {
        match register {
            Register::AX => &mut self.GP[0], Register::BX => &mut self.GP[1],
            Register::CX => &mut self.GP[2], Register::DX => &mut self.GP[3],
            Register::EAX => &mut self.GP[4], Register::EBX => &mut self.GP[5],
            Register::ECX => &mut self.GP[6], Register::EDX => &mut self.GP[7],
        }
    }
}

#[derive(Clone)]
///General Purpose Registers
enum GPRegister {
    AX(u8, u8), BX(u8, u8), CX(u8, u8),
    DX(u8, u8), EAX(u8, u8, u8, u8),
    EBX(u8, u8, u8, u8), ECX(u8, u8, u8, u8),
    EDX(u8, u8, u8, u8),
}

impl Debug for GPRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GPRegister::AX(a, b) => write!(f, "AX:\n   AL  AH\n   {:02X}  {:02X}\n", a, b),
            GPRegister::BX(a, b) => write!(f, "BX:\n   BL  BH\n   {:02X}  {:02X}\n", a, b),
            GPRegister::CX(a, b) => write!(f, "CX:\n   CL  CH\n   {:02X}  {:02X}\n", a, b),
            GPRegister::DX(a, b) => write!(f, "DX:\n   DL  DH\n   {:02X}  {:02X}\n", a, b),
            GPRegister::EAX(a, b, c, d) => write!(f, "EAX:\n    AL  AH  EAL  EAH\n     {:02X}  {:02X}  {:02X}   {:02X}\n", a, b, c, d),
            GPRegister::EBX(a, b, c, d) => write!(f, "EBX:\n    BL  BH  EBL  EBH\n     {:02X}  {:02X}  {:02X}   {:02X}\n", a, b, c, d),
            GPRegister::ECX(a, b, c, d) => write!(f, "ECX:\n    CL  CH  ECL  ECH\n     {:02X}  {:02X}  {:02X}   {:02X}\n", a, b, c, d),
            GPRegister::EDX(a, b, c, d) => write!(f, "EDX:\n    DL  DH  EDL  EDH\n     {:02X}  {:02X}  {:02X}   {:02X}\n", a, b, c, d),
        }
    }
}

impl GetValue<u32> for GPRegister {
    fn get_value(&self) -> u32 {
        match self {
            GPRegister::AX(a, b) | GPRegister::BX(a, b) | GPRegister::CX(a, b) |
            GPRegister::DX(a, b) => u16::from_le_bytes([*a, *b]) as u32,
            GPRegister::EAX(a, b, c, d) | GPRegister::EBX(a, b, c, d) | GPRegister::ECX(a, b, c, d) |
            GPRegister::EDX(a, b, c, d) => u32::from_le_bytes([*a, *b, *c, *d]),
        }
    }
}

impl SetValue<Data, ()> for GPRegister {
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
            SPRegister::SP(a, b) | SPRegister::BP(a, b) |
            SPRegister::IP(a, b) => u16::from_le_bytes([*a, *b]) as u32,
        }
    }
}

impl SetValue<Data, ()> for SPRegister {
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
            FLAGS::AF(a) | FLAGS::ZF(a) | FLAGS::SF(a) | FLAGS::TF(a) |
            FLAGS::IF(a) | FLAGS::DF(a) | FLAGS::OF(a) | FLAGS::CF(a) |
            FLAGS::PF(a) => *a,
        }
    }
}

impl SetValue<u8, ()> for FLAGS {
    fn set_value(&mut self, value: u8) {
        match self {
            FLAGS::ZF(a) | FLAGS::SF(a) | FLAGS::TF(a) | FLAGS::IF(a) |
            FLAGS::DF(a) | FLAGS::OF(a) | FLAGS::CF(a) | FLAGS::PF(a) |
            FLAGS::AF(a) => *a = value,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
///! Instruction Set. This is the set of instructions that the CPU can execute.
/// NB: Not all instructions are implemented.
enum IS {
    Mov, Add, Sub,
    Mul, Div, And,
    Or, Xor, Not,
    Syscall
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

// TODO: Implement SetValue for Data to cater for the u32 without data loss
impl SetValue<u32, Data> for Data {
    fn set_value(&mut self, value: u32)-> Self {
        match self {
            Data::Byte(data) => {
                *data = value as u8;
                Data::Byte(*data)
            },
            Data::Word(data) => {
                *data = value as u16;
                Data::Word(*data)
            },
            Data::Dword(data) => {
                *data = value;
                Data::Dword(*data)
            },
        }
    }
}

#[derive(Debug, Clone)]
enum MemOp {
    ///Memory address. This is interpreted as ```[label]``` 
    /// # Example:
    /// 
    /// this
    ///  ```
    /// Instruction::new(
    ///     IS::Mov, vec![Operand::Register(Register::AX), Operand::Immediate((Data::Word(0x00FF))]
    /// );
    /// Instruction
    ///     IS::Mov, vec![Operand::Memory(MemOp::Address("label".to_owned())), Operand::Register(Register::AX)]
    /// );
    /// ``` 
    /// is interpreted as
    /// ```
    /// mov ax, 0x00FF
    /// mov [label], ax
    /// ```
    Address(String),

    ///Value. This is interpreted as `data/raw value`
    /// # Example:
    /// 
    /// this 
    /// ```
    /// Instruction::new(
    ///     IS::Mov, vec![Operand::Register(Register::AX), Operand::Immediate((Data::Word(0x00FF))]
    /// );
    /// Instruction::new(
    ///     IS::Mov, vec![Operand::Memory(MemOp::Value("address".to_owned())), Operand::Register(Register::AX)]
    /// );
    /// ``` 
    /// is interpreted as
    /// ```
    /// mov ax, 0x00FF
    /// mov address, ax
    /// ```
    /// This would lead to an error as the first operand is expected to be an address/memory location
    Label(String),
}

#[derive(Debug, Clone)]
/// Operand type used to store operands for instructions
/// 
/// Usage example:
/// ```
/// Instruction::new(
///     IS::Mov, vec![Operand::Register(Register::AX), Operand::Immediate(Data::Word(0x00FF))]
/// );
/// ```
/// This example moves the value 0x00FF to the AX register
/// 
/// It simulates the instruction `MOV AX, 0x00FF` in x86 assembly
/// ```
enum Operand {
    Register(Register),
    Memory(MemOp),
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
enum ALUMode {
    Add, Sub, Mul,
    Div, And, Or,
    Xor, Not, Off
}

#[derive(Debug)]
/// Arithmetic Logic Unit.
/// 
/// This is the unit that performs arithmetic and logical operations.
/// 
/// All operations assume u8 values.
struct ALU{
    buffer: (u32, u32),
    mode: ALUMode,
}

impl ALU {
    fn new() -> ALU {
        ALU {
            buffer: (0, 0),
            mode: ALUMode::Off,
        }
    }

    /// Sets the mode of the ALU's operation state
    fn set_mode(&mut self, mode: ALUMode) {
        self.mode = mode;
    }

    fn operand_fetch(&mut self, destination: u32, source: u32) {
        self.buffer = (destination, source);
    }

    /// Executes the operation based on the mode of the ALU
    fn execute(&mut self) -> (u32, bool) {
        match self.mode {
            ALUMode::Add => self.add(),
            ALUMode::Sub => self.sub(),
            ALUMode::Off => panic!("ALU is off"),
            _ => panic!("Unsupported mode not implemented"),
        }
    }

    /// Adds the bytes(u8) in buffer of Alu and returns the result and a boolean indicating if there was an overflow
    /// Returns the sum as u32 and bool representation of overflow sign
    fn add(&mut self) -> (u32, bool) {
        self.buffer.0.overflowing_add(self.buffer.1)
    } 

    /// Subtracts two u8 values and returns the result and a boolean indicating if there was an overflow
    fn sub(&mut self) -> (u32, bool) {
        self.buffer.0.overflowing_sub(self.buffer.1)
    }
}

#[derive(Debug)]
/// Random Access Memory.
/// 
/// This is the unit that stores data of the running program.
struct RAM{
    data: Vec<u8>,
    capacity: usize,
}

impl RAM {
    fn new() -> RAM {
        RAM {
            data: Vec::with_capacity(1024),
            capacity: 1024,
        }
    }
}

#[derive(Debug)]
/// Memory Unit.
/// 
/// This is the unit that stores data and code sections.
/// 
/// It is used to simulate the memory of the CPU.
struct MemoryUnit {
    ///Data section of the memory unit. 
    /// 
    ///It stores program variables in the form of key(label)-value(memory address) pairs.
    /// 
    data_section: HashMap<String, Data>,
    ///Code section of the memory unit.
    /// 
    ///It stores the program instructions.
    code_section: Vec<Instruction>,
    ///Memory Access bus.
    data_bus: RAM
}

/// Implementation of the Memory Unit that manages data used by the CPU and running program.
/// 
/// It contains the data and code sections of the program and does the read and write operations to main memory.
// TODO: Implement the MemoryUnit's read and write methods to cater for different data sizes
impl MemoryUnit {
    fn new(data_section: HashMap<String, Data>, code_section: Vec<Instruction>) -> MemoryUnit {
        MemoryUnit {
            data_section,
            code_section,
            data_bus: RAM::new(),
        }
    }

    fn get_mem_capacity(&self) -> usize {
        self.data_bus.capacity
    }

    fn get_data_len(&self) -> usize {
        self.data_bus.data.len()
    }

    /// Reads data from the main memory.
    /// 
    /// Address is a 32 bit integer that contains the actual index of required bytes in the RAM Vec as data and the length of data to be read.
    /// 
    /// Address = 16 bit actual address + 16 bit length of data to be read.
    fn read_data(&self, address: Data) -> Vec<u8> {
        let address_value = address.get_value();
        match address {
            Data::Byte(_) => {
                if self.get_data_len() < 1 {
                    panic!("Memory is empty");
                }
                let actual_address = address_value >> 4;
                let length = address_value & 0x000F;
                self.data_bus.data[actual_address as usize..(actual_address + length) as usize].to_vec()
            },
            Data::Word(_) => {
                if self.get_data_len() < 2 {
                    panic!("Memory is empty");
                }
                let actual_address = address_value >> 8;
                let length = address_value & 0x00FF;
                self.data_bus.data[actual_address as usize..(actual_address + length) as usize].to_vec()
            },
            Data::Dword(_) => {
                if self.get_data_len() < 4 {
                    panic!("Memory is empty");
                }
                let actual_address = address_value >> 16;
                let length = address_value & 0xFFFF;
                self.data_bus.data[actual_address as usize..(actual_address + length) as usize].to_vec()
            }
        }
    }

    /// Writes data to the main memory.
    /// 
    /// Address is a 32 bit integer that contains the actual index of required bytes in the RAM Vec as data and the length of data to be written.
    /// 
    /// Data is the bytes to be written to memory.
    /// 
    /// This operation assumes constant data size and doesn't reallocate memory for data exceeding initial data size.
    fn write_data(&mut self, address: Data, data: Vec<u8>) {
        let address_value = address.get_value();
        let mut actual_address = 0;
        let mut length = 0;

        match address {
            Data::Byte(_) => {
                if self.get_data_len() < 1 {
                    panic!("Memory is empty");
                }
                actual_address = address_value >> 4;
                length = address_value & 0x000F;
            },
            Data::Word(_) => {
                if self.get_data_len() < 2 {
                    panic!("Memory is empty");
                }
                actual_address = address_value >> 8;
                length = address_value & 0x00FF;
            },
            Data::Dword(_) => {
                if self.get_data_len() < 4 {
                    panic!("Memory is empty");
                }
                actual_address = address_value >> 16;
                length = address_value & 0xFFFF;
            },
        }
        // If the actual address is greater than the length of the data in memory, extend the memory by writing new data.
        if actual_address as usize > self.get_data_len()-1 {
            if self.get_mem_capacity() == 0 {
                panic!("Memory is full");
            }
            self.data_bus.data.extend(data);
        }
        else {
            // If the actual address is less than the length of the data in memory, re-writes the existing data at the specified address with the new data.
            self.data_bus.data[actual_address as usize..(actual_address + data.len() as u32) as usize].copy_from_slice(&data);

            // If the data length is less than the length of the data bus, fill the remaining space with 0.
            if data.len() < length as usize {
                self.data_bus.data[actual_address as usize + data.len()..(actual_address + length) as usize].fill(0);
            }
        }
    }
}

#[derive(Debug)]
/// Central Processing Unit.
/// 
/// This is the main unit that controls the execution of the program.
/// 
/// It contains the ALU, Registers and Memory Unit.
// TODO: Implement the CPU's store_label_data method to cater for different data sizes
struct CPU {
    alu: ALU,
    registers: Registers,
    flags: [FLAGS; 9],
    memory_unit: MemoryUnit,
}

impl CPU {
    fn new(data_section: HashMap<String, Data>, code_section: Vec<Instruction>)-> CPU {
        let mut cpu = CPU {
            alu: ALU::new(),
            registers: Registers {
                GP: [GPRegister::AX(0, 0), GPRegister::BX(0, 0), GPRegister::CX(0, 0), GPRegister::DX(0, 0), GPRegister::EAX(0, 0, 0, 0), GPRegister::EBX(0, 0, 0, 0), GPRegister::ECX(0, 0, 0, 0), GPRegister::EDX(0, 0, 0, 0)],
                SP: [SPRegister::SP(0, 0), SPRegister::BP(0, 0), SPRegister::IP(0, 0)],
            },
            flags: [FLAGS::PF(0), FLAGS::AF(0), FLAGS::ZF(0), FLAGS::SF(0), FLAGS::TF(0), FLAGS::IF(0), FLAGS::DF(0), FLAGS::OF(0), FLAGS::CF(0)],
            memory_unit: MemoryUnit {
                data_section,
                code_section,
                data_bus: RAM::new(),
            },
        };
        cpu.store_label_data();
        cpu
    }

    #[allow(dead_code)]
    fn preview_flags(&self){
        println!("Flags:");
        self.flags.iter().for_each(|flag| {
            println!("{:?}", flag);
        });
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

    // Address is a 32 bit integer that contains the actual index of required bytes in the RAM Vec as data and the length of data to be read.
    // Address = 16 bit actual address + 16 bit length of data to be read.
    fn store_label_data(&mut self) {
        let mut required_capacity = 0;
    
        // Calculate required capacity first
        for (_, data) in self.memory_unit.data_section.iter() {
            required_capacity += match data {
                Data::Byte(_) => 1,
                Data::Word(_) => 2,
                Data::Dword(_) => 4,
            };
        }
    
        // Check if we have enough space in data_bus
        if self.memory_unit.data_bus.capacity < required_capacity {
            panic!("Not enough capacity in data bus!");
        }
    
        // Store data
        for (i, (_, data)) in self.memory_unit.data_section.iter_mut().enumerate() {
            match data {
                Data::Byte(value) => {
                    let address = (1 << 4) | (i as u8);
                    self.memory_unit.data_bus.data.push(*value);
                    self.memory_unit.data_bus.capacity -= 1;
                    data.set_value(address as u32);
                    println!("Stored address: {:?}", data);
                }
                Data::Word(value) => {
                    let bytes = value.to_le_bytes();
                    let address = (2 << 8) | (i as u16);
                    self.memory_unit.data_bus.data.extend(&bytes);
                    self.memory_unit.data_bus.capacity -= 2;
                    data.set_value(address as u32);
                    println!("Stored address: {:?}", data);
                }
                Data::Dword(value) => {
                    let bytes = value.to_le_bytes();
                    let address = (4 << 16) | (i as u32);
                    self.memory_unit.data_bus.data.extend(&bytes);
                    self.memory_unit.data_bus.capacity -= 4;
                    data.set_value(address);
                    println!("Stored address: {:?}", data);
                }
            }
        }
    }
    

    /// The fetch stage operation of CPU's workflow.
    fn fetch(&mut self) {
            let pc = self.registers.SP[2].get_value();
            let instruction = self.memory_unit.code_section[pc as usize].clone();
            self.registers.SP[2].set_value(Data::Word((pc + 1) as u16));
            self.decode(instruction);
        }

    /// The decode stage operation of CPU's workflow.
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
                        let src_value = self.registers.get_register(src_register.clone()).get_value();
                        let dest_reg = self.registers.get_register(dest_register.clone());
                        match dest_reg {
                            GPRegister::AX(_, _) | GPRegister::BX(_, _) | GPRegister::CX(_, _) |
                            GPRegister::DX(_, _) => dest_reg.set_value(Data::Word(src_value as u16)),
                            GPRegister::EAX(_, _, _, _) | GPRegister::EBX(_, _, _, _) | GPRegister::ECX(_, _, _, _) |
                            GPRegister::EDX(_, _, _, _) => dest_reg.set_value(Data::Dword(src_value)),
                        }
                        println!("Data movement occured:\nRegister: {0:?} -> Register: {1:?}\nRegister {1:?} updated to: \n{2:?}", src_register, dest_register, dest_reg);
                    },
                    (Operand::Register(register), Operand::Memory(operand)) => {
                        let mut src_value_address = 0;

                        // Extract the data from memory if the operand is an address
                        // Extract the memory address from the data section if the operand is a label
                        match operand {
                            MemOp::Address(label) => {
                                match self.memory_unit.data_section.get(&label) {
                                    Some(value) => {
                                        let mut data: Vec<u8> = vec![];
                                        match value {
                                            Data::Byte(_) => {
                                                data = self.memory_unit.read_data(value.clone());
                                                src_value_address = u8::from_le_bytes(data.as_slice().try_into().unwrap()) as u32;
                                            },
                                            Data::Word(_) => {
                                                data = self.memory_unit.read_data(value.clone());
                                                match data.as_slice() {
                                                    [a, b] => {
                                                        src_value_address = u16::from_le_bytes([*a, *b]) as u32;
                                                    }
                                                    [a] => {
                                                        src_value_address = u16::from_le_bytes([*a, 0]) as u32;
                                                    }
                                                    _ => {
                                                        println!("Address: {:?}\nData: {:?}\nMemory: {:?}", value.get_value(), data, self.memory_unit.data_bus.data);
                                                        panic!("Data slice: {:?}", data.as_slice());
                                                    }
                                                }
                                            },
                                            Data::Dword(_) => {
                                                data = self.memory_unit.read_data(value.clone());
                                                src_value_address = u32::from_le_bytes(data.as_slice().try_into().unwrap());
                                            }
                                        }
                                    }
                                    None => {
                                        println!("Use of undeclared memory address: [{:?}]", label);
                                        panic!("Invalid memory address at {:?}", instruction);
                                    }
                                }
                            }
                            MemOp::Label(data) => {
                                match self.memory_unit.data_section.get(&data) {
                                    Some(value) => {
                                        src_value_address = value.get_value();
                                    }
                                    None => {
                                        println!("Use of undeclared lable: {:?}", data);
                                        panic!("Invalid label usage at {:?}", instruction);
                                    }
                                }
                            }
                        };
                        
                        let dest_reg = self.registers.get_register(register.clone());
                        match dest_reg {
                            GPRegister::AX(_, _) | GPRegister::BX(_, _) | GPRegister::CX(_, _) |
                            GPRegister::DX(_, _) => dest_reg.set_value(Data::Word(src_value_address as u16)),
                            GPRegister::EAX(_, _, _, _) | GPRegister::EBX(_, _, _, _) | GPRegister::ECX(_, _, _, _) |
                            GPRegister::EDX(_, _, _, _) => dest_reg.set_value(Data::Dword(src_value_address)),
                        }
                        println!("Data movement occured:\nMemory address: {0:?} -> Register: {1:?}\nRegister {1:?} updated to: \n{2:?}", src_value_address, register, dest_reg);
                    },

                    // Create address for the value, store the address in data_section, store the value in memory and address in the register
                    (Operand::Register(register), Operand::Immediate(value)) => {
                        let data = value.get_value();
                        let dest_reg = self.registers.get_register(register.clone());
                        match dest_reg {
                            GPRegister::AX(_, _) | GPRegister::BX(_, _) | GPRegister::CX(_, _) |
                            GPRegister::DX(_, _) => dest_reg.set_value(Data::Word(data as u16)),
                            GPRegister::EAX(_, _, _, _) | GPRegister::EBX(_, _, _, _) | GPRegister::ECX(_, _, _, _) |
                            GPRegister::EDX(_, _, _, _) => dest_reg.set_value(Data::Dword(data)),
                        }
                        println!("Data movement occured:\nImmediate value: {0:?} -> Register: {1:?}\nRegister {1:?} updated to: \n{2:?}", value, register, dest_reg);
                    },
                    (Operand::Memory(operand), Operand::Register(register)) => {
                        let src_value = self.registers.get_register(register.clone()).get_value();

                        let label = match operand {
                            MemOp::Address(label) => {
                                label
                            }
                            MemOp::Label(data) => {
                                println!("Invalid memory address: {:?} at instruction {:?}", data, instruction);
                                panic!("Expected an address/memory location, found a value");
                            }
                        };

                        // Check if the memory address exists in the data section
                        if let None = self.memory_unit.data_section.get_mut(&label) {
                           println!("Use of undeclared memory address: {:?}", label);
                           panic!("Invalid memory address at {:?}", instruction);
                        }

                        // Extract the data from the register to store in the memory address
                        let data = match self.registers.get_register(register.clone()) {
                            GPRegister::AX(_, _) | GPRegister::BX(_, _) | GPRegister::CX(_, _) | 
                            GPRegister::DX(_, _) => Data::Word(src_value as u16),
                            GPRegister::EAX(_, _, _, _) | GPRegister::EBX(_, _, _, _) | GPRegister::ECX(_, _, _, _) |
                            GPRegister::EDX(_, _, _, _) => Data::Dword(src_value),
                        };

                        let address = self.memory_unit.data_section[&label].clone();
                        self.memory_unit.write_data(address, data.get_value().to_le_bytes().to_vec());
                        println!("Data movement occured:\nRegister: {0:?} -> Memory address: [{1:?}]\nMemory address {1:?} updated to: \n{2:?}\n", register, label, data.get_value());
                    },
                    (Operand::Memory(operand), Operand::Immediate(value)) => {
                        let label = match operand {
                            MemOp::Address(label) => {
                                label
                            }
                            MemOp::Label(data) => {
                                println!("Invalid memory address: {:?} at instruction {:?}", data, instruction);
                                panic!("Expected an address/memory location, found a value");
                            }
                        };
                        if let None = self.memory_unit.data_section.get_mut(&label) {
                            println!("Use of undeclared memory address: {:?}", label);
                            panic!("Invalid memory address at {:?}", instruction);
                        }
                        let address = self.memory_unit.data_section[&label].clone();
                        self.memory_unit.write_data(address, value.get_value().to_le_bytes().to_vec());
                        println!("Data movement occured:\nImmediate value: {0:?} -> Memory address: [{1:?}]\nMemory address [{1:?}] updated to: \n{0:?}\n", value, label);
                    },
                    _ => {
                        panic!("Invalid operands for MOV instruction at {0:?} Be sure that:\n1. Immediate value isn't used as destination.\n2. Movement from memory to memory aren't possible{0:?}", instruction);
                    }
                }
            },
            IS::Add => {
                match instruction.verify_operands() {
                    false => {
                        panic!("Invalid operands for ADD instruction at {0:?} ADD expects only 2 operands", instruction);
                    },
                    _ => self.alu.set_mode(ALUMode::Add)
                }

                let dest = instruction.operands[0].clone();
                let src = instruction.operands[1].clone();
                match (dest, src) {
                    (Operand::Register(dest_register), Operand::Register(src_register)) => {
                        let src_value = self.registers.get_register(src_register.clone()).get_value();
                        let dest_reg = self.registers.get_register(dest_register.clone());
                        let dest_value = dest_reg.get_value();

                        self.alu.operand_fetch(dest_value, src_value);

                        let (result, overflow) = self.alu.execute();

                        match src_register {
                            Register::AX | Register::BX | 
                            Register::CX | Register::DX=> dest_reg.set_value(Data::Word(result as u16)),
                            Register::EAX | Register::EBX |
                            Register::ECX | Register::EDX => dest_reg.set_value(Data::Dword(result)),
                        }

                        match overflow {
                            true => self.flags[7].set_value(1),
                            false => self.flags[7].set_value(0),
                        }
                        println!("Data addition occured:\nRegister: {0:?} + Register: {1:?}\nRegister {1:?} updated to: \n{2:?}", dest_register, src_register, dest_reg);
                    },
                    (Operand::Register(register), Operand::Memory(operand)) => {
                        let (label, address) = match operand {
                            MemOp::Address(label) => {
                                match self.memory_unit.data_section.get(&label) {
                                    Some(value) => {
                                        (label, value)
                                    }
                                    None => {
                                        println!("Use of undeclared memory address: [{:?}]", label);
                                        panic!("Invalid memory address at {:?}", instruction);
                                    }
                                }
                            }
                            MemOp::Label(data) => {
                                println!("Invalid memory address: {:?} at instruction {:?}", data, instruction);
                                panic!("Expected an address/memory location, found a value");
                            }
                        };

                        let dest_reg = self.registers.get_register(register.clone());
                        let dest_value = dest_reg.get_value();
                        let src_data = self.memory_unit.read_data(address.clone());
                        let src_value = u32::from_le_bytes(src_data.as_slice().try_into().unwrap());

                        self.alu.operand_fetch(dest_value, src_value);

                        let (result, overflow) = self.alu.execute();

                        match address {
                            Data::Byte(_) => dest_reg.set_value(Data::Byte(result as u8)),
                            Data::Word(_) => dest_reg.set_value(Data::Word(result as u16)),
                            Data::Dword(_) => dest_reg.set_value(Data::Dword(result)),
                        }

                        match overflow {
                            true => self.flags[7].set_value(1),
                            false => self.flags[7].set_value(0),
                        }
                        println!("Data addition occured:\nMemory address: [{0:?}] + Register: {1:?}\nRegister {1:?} updated to: \n{2:?}", label, register, dest_reg);
                    },
                    (Operand::Register(register), Operand::Immediate(value)) => {
                        let dest_reg = self.registers.get_register(register.clone());
                        let dest_value = dest_reg.get_value();

                        let mut operand_bytes = Vec::from(dest_value.to_le_bytes());
                        operand_bytes.extend(value.get_value().to_le_bytes());
                        self.alu.operand_fetch(dest_value, value.get_value());

                        let (result, overflow) = self.alu.execute();

                        match value {
                            Data::Byte(_) => dest_reg.set_value(Data::Byte(result as u8)),
                            Data::Word(_) => dest_reg.set_value(Data::Word(result as u16)),
                            Data::Dword(_) => dest_reg.set_value(Data::Dword(result)),
                        }

                        match overflow {
                            true => self.flags[7].set_value(1),
                            false => self.flags[7].set_value(0),
                        }
                        println!("Data addition occured:\nImmediate value: {0:?} + Register: {1:?}\nRegister {1:?} updated to: \n{2:?}", value, register, dest_reg);
                    },
                    (Operand::Memory(operand), Operand::Register(register)) => {
                        let src_value = self.registers.get_register(register.clone()).get_value();

                        let data_section =self.memory_unit.data_section.clone();
                        let address = match operand {
                            MemOp::Address(label) => {
                                match data_section.get(&label) {
                                    Some(value) => {
                                        value
                                    }
                                    None => {
                                        println!("Use of undeclared memory address: [{:?}]", label);
                                        panic!("Invalid memory address at {:?}", instruction);
                                    }
                                }
                            },
                            MemOp::Label(data) => {
                                println!("Invalid memory address: {:?} at instruction {:?}", data, instruction);
                                panic!("Expected an address/memory location, found a value");
                            },
                        };
                        let addr_data = self.memory_unit.read_data(address.clone());
                        let addr_value = u32::from_le_bytes(addr_data.as_slice().try_into().unwrap());
                        self.alu.operand_fetch(addr_value, src_value);
                        let (result, overflow) = self.alu.execute();

                        let address_clone = address.clone();
                        self.memory_unit.write_data(address_clone, result.to_le_bytes().to_vec());

                        match overflow {
                            true => self.flags[7].set_value(1),
                            false => self.flags[7].set_value(0),
                        }

                        println!("Data addition occured:\nMemory address value: [{0:?}]: {3:?} + Register: {2:?}\nMemory address [{0:?}] updated to: \n{1:?}", address.get_value(), result, register, addr_value);
                            
                    },
                    (Operand::Memory(operand), Operand::Immediate(value)) => {
                        let src_value = value.get_value();

                        let (address, label) = match operand {
                            MemOp::Address(label) => {
                                match self.memory_unit.data_section.get(&label) {
                                    Some(value) => {
                                        (value, label)
                                    }
                                    None => {
                                        println!("Use of undeclared memory address: [{:?}]", label);
                                        panic!("Invalid memory address at {:?}", instruction);
                                    }
                                }
                            },
                            MemOp::Label(data) => {
                                println!("Invalid memory address: {:?} at instruction {:?}", data, instruction);
                                panic!("Expected an address/memory location, found a value");
                            }
                        };

                        let addr_data = self.memory_unit.read_data(address.clone());
                        let addr_value = u32::from_le_bytes(addr_data.as_slice().try_into().unwrap());

                        self.alu.operand_fetch(addr_value, src_value);
                        let (result, overflow) = self.alu.execute();

                        match overflow {
                            true => self.flags[7].set_value(1),
                            false => self.flags[7].set_value(0),
                        }

                        println!("Data addition occured:\nMemory address value: [{0:?}]: {3:?} + Immediate value: {2:?}\nMemory address [{0:?}] updated to: \n{1:?}", label, result, src_value, addr_value);
                    },
                    _ => {
                        panic!("Invalid operands for ADD instruction at {0:?} Be sure that:\n1. Immediate value isn't used as destination.\n2. Movement from memory to memory aren't possible{0:?}", instruction);
                    }
                }
                self.alu.set_mode(ALUMode::Off);
            },
            IS::Sub => {
                match instruction.verify_operands() {
                    false => {
                        panic!("Invalid operands for SUB instruction at {0:?} SUB expects only 2 operands", instruction);
                    },
                    _ => self.alu.set_mode(ALUMode::Sub)
                }

                let dest = instruction.operands[0].clone();
                let src = instruction.operands[1].clone();
                match (dest, src) {
                    (Operand::Register(dest_register), Operand::Register(src_register)) => {
                        let src_value = self.registers.get_register(src_register.clone()).get_value();
                        let dest_reg = self.registers.get_register(dest_register.clone());
                        let dest_value = dest_reg.get_value();

                        self.alu.operand_fetch(dest_value, src_value);

                        let (result, overflow) = self.alu.execute();

                        match src_register {
                            Register::AX | Register::BX | 
                            Register::CX | Register::DX=> dest_reg.set_value(Data::Word(result as u16)),
                            Register::EAX | Register::EBX |
                            Register::ECX | Register::EDX => dest_reg.set_value(Data::Dword(result)),
                        }

                        match overflow {
                            true => self.flags[7].set_value(1),
                            false => self.flags[7].set_value(0),
                        }
                        println!("Subtraction occured:\nRegister: {0:?} - Register: {1:?}\nRegister {1:?} updated to: \n{2:?}", dest_register, src_register, dest_reg);
                    },
                    (Operand::Register(register), Operand::Memory(operand)) => {

                        let (address, src_value, label) = match operand {
                            MemOp::Address(label) => {
                                match self.memory_unit.data_section.get(&label) {
                                    Some(value) => {
                                        let src_value = self.memory_unit.read_data(value.clone());
                                        (value, u32::from_le_bytes(src_value.as_slice().try_into().unwrap()), label)
                                    }
                                    None => {
                                        println!("Use of undeclared memory address: [{:?}]", label);
                                        panic!("Invalid memory address at {:?}", instruction);
                                    }
                                }
                            },
                            MemOp::Label(data) => {
                                println!("Invalid memory address: {:?} at instruction {:?}", data, instruction);
                                panic!("Expected an address/memory location, found a value");
                            },
                        };

                        let dest_reg = self.registers.get_register(register.clone());
                        let dest_value = dest_reg.get_value();

                        self.alu.operand_fetch(dest_value, src_value);

                        let (result, overflow) = self.alu.execute();

                        match address {
                            Data::Byte(_) => dest_reg.set_value(Data::Byte(result as u8)),
                            Data::Word(_) => dest_reg.set_value(Data::Word(result as u16)),
                            Data::Dword(_) => dest_reg.set_value(Data::Dword(result)),
                        }

                        match overflow {
                            true => self.flags[7].set_value(1),
                            false => self.flags[7].set_value(0),
                        }
                        println!("Subtraction occured:\nMemory address: [{0:?}] - Register: {1:?}\nRegister {1:?} updated to: \n{2:?}", label, register, dest_reg);
                    },
                    (Operand::Register(register), Operand::Immediate(value)) => {
                        let dest_reg = self.registers.get_register(register.clone());
                        let dest_value = dest_reg.get_value();

                        let mut operand_bytes = Vec::from(dest_value.to_le_bytes());
                        operand_bytes.extend(value.get_value().to_le_bytes());
                        self.alu.operand_fetch(dest_value, value.get_value());

                        let (result, overflow) = self.alu.execute();

                        match value {
                            Data::Byte(_) => dest_reg.set_value(Data::Byte(result as u8)),
                            Data::Word(_) => dest_reg.set_value(Data::Word(result as u16)),
                            Data::Dword(_) => dest_reg.set_value(Data::Dword(result)),
                        }

                        match overflow {
                            true => self.flags[7].set_value(1),
                            false => self.flags[7].set_value(0),
                        }
                        println!("Subtraction occured:\nImmediate value: {0:?} - Register: {1:?}\nRegister {1:?} updated to: \n{2:?}", value, register, dest_reg);
                    },
                    (Operand::Memory(operand), Operand::Register(register)) => {
                        let src_value = self.registers.get_register(register.clone()).get_value();

                        let (address_value, label) = match operand {
                            MemOp::Address(label) => {
                                match self.memory_unit.data_section.get(&label) {
                                    Some(value) => {
                                        let addr_data = self.memory_unit.read_data(value.clone());
                                        (u32::from_le_bytes(addr_data.as_slice().try_into().unwrap()), label)
                                    }
                                    None => {
                                        println!("Use of undeclared memory address: [{:?}]", label);
                                        panic!("Invalid memory address at {:?}", instruction);
                                    }
                                }
                            },
                            MemOp::Label(data) => {
                                println!("Invalid memory address: {:?} at instruction {:?}", data, instruction);
                                panic!("Expected an address/memory location, found a value");
                            },
                        };
                        
                        self.alu.operand_fetch(src_value, src_value);
                        let (result, overflow) = self.alu.execute();

                        match overflow {
                            true => self.flags[7].set_value(1),
                            false => self.flags[7].set_value(0),
                        }

                        println!("Subtraction occured:\nMemory address value: [{0:?}]: {1:?} - Register: {2:?}\nMemory address [{0:?}] updated to: \n{3:?}", label, address_value, register, result);
                    },
                    (Operand::Memory(operand), Operand::Immediate(value)) => {
                        let src_value = value.get_value();

                        let (addr_value, label) = match operand {
                            MemOp::Address(label) => {
                                match self.memory_unit.data_section.get(&label) {
                                    Some(value) => {
                                        let addr_data = self.memory_unit.read_data(value.clone());
                                        match value {
                                            Data::Byte(_) => (u8::from_le_bytes(addr_data.as_slice().try_into().unwrap()) as u32, label),
                                            Data::Word(_) => {
                                                match addr_data.as_slice() {
                                                    [a, b] => (u16::from_le_bytes([*a, *b]) as u32, label),
                                                    [a] => (u16::from_le_bytes([*a, 0]) as u32, label),
                                                    _ => {
                                                        panic!("Data slice: {:?}", addr_data.as_slice());
                                                    }
                                                }
                                            },
                                            Data::Dword(_) => (u32::from_le_bytes(addr_data.as_slice().try_into().unwrap()), label)
                                            
                                        }
                                    }
                                    None => {
                                        println!("Use of undeclared memory address: [{:?}]", label);
                                        panic!("Invalid memory address at {:?}", instruction);
                                    }
                                }
                            }
                            MemOp::Label(data) => {
                                println!("Invalid memory address: {:?} at instruction {:?}", data, instruction);
                                panic!("Expected an address/memory location, found a value");
                            }
                        };

                        self.alu.operand_fetch(addr_value, src_value);
                        let (result, overflow) = self.alu.execute();

                        match overflow {
                            true => self.flags[7].set_value(1),
                            false => self.flags[7].set_value(0),
                        }

                        println!("Subtraction occured:\nMemory address value: [{0:?}]: {3:?} - Immediate value: {2:?}\nMemory address [{0:?}] updated to: \n{1:?}", label, result, src_value, result);
                    },
                    _ => {
                        panic!("Invalid operands for SUB instruction at {0:?} Be sure that:\n1. Immediate value isn't used as destination.\n2. Movement from memory to memory aren't possible{0:?}", instruction);
                    }
                }
                self.alu.set_mode(ALUMode::Off);
            },
            IS::Syscall => {
                match instruction.verify_operands() {
                    false => {
                        panic!("Invalid operands for SYSCALL instruction at {0:?} SYSCALL doesn't take any operands", instruction);
                    },
                    _ => {}
                }
                match self.syscall() {
                    Ok(_) => {},
                    Err(err) => {
                        let description = format!("Error while running Syscall instruction: {:?}\nReason: {:?}", instruction, err);
                        panic!("{}", description)
                    },
                }
            },

            _ => panic!("Unsupported Instruction at {:?}", instruction),
        }
    }

    fn syscall(&mut self)-> Result<(), String> {
        let syscall_number: u8 = self.registers.get_register(Register::AX).get_value() as u8;
        let file_descriptor: u8 = self.registers.get_register(Register::BX).get_value() as u8;
        let data_length: u16  = self.registers.get_register(Register::DX).get_value() as u16;
        let address_register = self.registers.get_register(Register::CX);
        let address = match address_register {
            GPRegister::AX(_, _) | GPRegister::BX(_, _) | GPRegister::CX(_, _) |
            GPRegister::DX(_, _) => Data::Dword(address_register.get_value()),
            GPRegister::EAX(_, _, _, _) | GPRegister::EBX(_, _, _, _) | GPRegister::ECX(_, _, _, _) |
            GPRegister::EDX(_, _, _, _) => Data::Dword(address_register.get_value()),
        };

        // Address is packaged as 32 bit number with the upper 16 bits representing the lenght of data, lower 16 bits hold the actual address of data in memory
        match syscall_number {
            // Read from file descriptor(file or keyboard)
            // Currently supports only keyboard input
            1 => {
                let mut read_buffer = vec![0; data_length as usize];
                stdin().read_exact(read_buffer.as_mut_slice()).unwrap();

                // 
                self.memory_unit.write_data(address.clone(), read_buffer);
                self.registers.get_register(Register::CX).set_value(address);
                Ok(())
            },
            // Write to file descriptor(file or screen)
            // Currently supports only screen output
            2 => {
                let mut write_buffer = self.memory_unit.read_data(address);
                stdout().write_all(write_buffer.as_mut_slice()).unwrap();
                Ok(())
            }
            60 => {
                println!("Program exited with code: {}", file_descriptor);
                std::process::exit(file_descriptor as i32);
            }
            _ => {
                let err_msg = format!("Unknown file systemcall number: {}", syscall_number);
                Err(err_msg)
            }
        }
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
        Instruction::new(IS::Mov, vec![Operand::Register(Register::AX), Operand::Immediate(Data::Word(300))]),
        Instruction::new(IS::Mov, vec![Operand::Register(Register::BX), Operand::Memory(MemOp::Address("num".to_string()))]),
        Instruction::new(IS::Add, vec![Operand::Register(Register::CX), Operand::Register(Register::AX)]),
        Instruction::new(IS::Sub, vec![Operand::Register(Register::CX), Operand::Register(Register::BX)]),
        Instruction::new(IS::Mov, vec![Operand::Memory(MemOp::Address("result".to_string())), Operand::Register(Register::CX)]),
        Instruction::new(IS::Sub, vec![Operand::Memory(MemOp::Address("num2".to_string())), Operand::Immediate(Data::Word(0x000F))]),
    ];
    let mut cpu = CPU::new(data_section, code_section);
    cpu.run();
}