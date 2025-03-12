# Virtual CPU Project

## Overview
This project aims to build a **minimalist virtual CPU** with fundamental instructions that can be extended at any time. The goal is to provide a learning platform for **computer architecture, assembly language, and low-level programming concepts**.

A key feature of this project is the planned implementation of an **assembly syntax parser** that will act as the frontend, allowing developers to write and execute assembly instructions for the virtual CPU.

## Features
- **Register-based CPU architecture**
- **Memory management (.data, .bss, .text sections)**
- **Basic arithmetic and logical operations**
- **Extensible instruction set**
- **Assembly-like syntax for interaction**
- **Simple ALU for computations**
- **Flags register for condition handling**

## Planned Features
- **Full assembly parser**: Translate user-written ASM code into CPU instructions.
- **More instructions**: Implement branching (JMP, JZ, JNZ), bitwise operations, and stack operations.
- **I/O operations**: Simulate input/output handling.
- **Interactive REPL**: A shell-like interface for executing instructions dynamically.

## Getting Started
### Prerequisites
- **Rust** (latest stable version recommended)
- **Cargo** (Rust’s package manager)

### Installation
Clone the repository:
```sh
    git clone https://github.com/Aine-dickson/cpu.git
    cd cpu
```

### Building the Project
```sh
    cargo build --release
```

### Running the Virtual CPU
```sh
    cargo run
```

## Contributing
This project is designed to be **extensible**, and contributions are welcome! If you’d like to add new instructions, improve the ALU, or contribute to the assembly parser, feel free to:
- **Fork the repository**
- **Create a feature branch**
- **Submit a pull request**

### Contribution Guidelines
- Code should follow Rust best practices.
- Provide detailed documentation for new features.
- Ensure proper error handling in instruction execution.
- Add test cases for new instructions.

## License
This project is licensed under the **MIT License**.

## Acknowledgments
- Inspired by real CPU architectures and educational projects.
- Contributions from the open-source community are highly appreciated!

---

🚀 **Join the development and help build an educational virtual CPU!**

