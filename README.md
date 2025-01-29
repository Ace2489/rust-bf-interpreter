# Rust [BF](https://en.wikipedia.org/wiki/Brainfuck) Interpreter

[![Rust](https://img.shields.io/badge/Rust-1.58%2B-blue?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A low-level BF interpreter implementation in Rust, featuring memory-safe execution and efficient instruction processing.

## Features

- Full instruction set support (`>`, `<`, `+`, `-`, `.`, `,`, `[`, `]`)
- Custom program termination character (`;`)
- 30,000-cell memory tape (standard BF size)
- Panic-based error handling for invalid programs
- Input/output buffer management
- Nested loop support with bracket matching
- Zero external dependencies

## Installation

1. Ensure you have [Rust installed](https://www.rust-lang.org/tools/install) (1.58+)
2. Clone the repository:
   ```bash
   git clone https://github.com/ace2489/rust-bf-interpreter.git
   cd rust-bf-interpreter
   ```
3. Build with optimizations:
   ```bash
   cargo build --release
   ```

## Usage

```rust
use bf_interpreter::Machine;

fn main() {
    let program = ">>+++-.,[[]++];".to_string();
    let input = vec![1, 2, 3];
    
    let mut machine = Machine {
        instruction_pointer: 0,
        data: [0; 30000],
        data_pointer: 0,
        input,
        output: Vec::new(),
    };

    machine.run(program);
    println!("Final state: {:#?}", machine);
}
```

## Implementation Details

### Core Components

- **Machine State**:
  - `instruction_pointer`: Current command position
  - `data`: 30,000-byte memory array
  - `data_pointer`: Current memory cell position
  - Input/output buffers

### Execution Flow

1. Instruction parsing with bracket matching
2. Memory pointer manipulation
3. Arithmetic operations with overflow protection
4. I/O operations:
   - `.` writes to output buffer
   - `,` reads from input buffer (LIFO)
5. Loop handling with nested bracket resolution

### Error Handling

Panics occur for:
- Unmatched brackets
- Missing input data
- Invalid program termination
- Memory access errors

## Benchmark Example

Sample program execution:
```rust
let program = "++++++++[>++++++++<-]>.".to_string(); // Outputs 64
```

Expected output:
```rust
Machine {
    instruction_pointer: 15,
    data: [0, 64, 0, ...],
    data_pointer: 1,
    input: [],
    output: [64],
}
```

## Limitations[App Still in Development]

- Non-standard program termination requirement (`;`)
- Panic-based error handling (no graceful recovery)
- Input is consumed in reverse order (stack-based)
- No overflow protection for cell values (u8 wrapping)

## License

MIT License - See [LICENSE](LICENSE) for details
