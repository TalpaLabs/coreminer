<div align="center">
    <img alt="icon" src="./docs/img/logo.svg" width="60%"/>
    <h3>🔩 coreminer 👁️</h3>
    <p>
        Debug those programs that don't want to be
    </p>
    <br/>
    <a href="https://github.com/debugger-bs/coreminer/actions/workflows/release.yaml">
        <img src="https://img.shields.io/github/actions/workflow/status/debugger-bs/coreminer/release.yaml?label=Release" alt="Release Status"/>
    </a>
    <a href="https://github.com/debugger-bs/coreminer/actions/workflows/cargo.yaml">
        <img src="https://img.shields.io/github/actions/workflow/status/debugger-bs/coreminer/cargo.yaml?label=Rust%20CI" alt="Rust CI"/>
    </a>
    <a href="https://github.com/debugger-bs/coreminer/blob/master/LICENSE">
        <img src="https://img.shields.io/github/license/debugger-bs/coreminer" alt="License"/>
    </a>
    <a href="https://github.com/debugger-bs/coreminer/releases">
        <img src="https://img.shields.io/github/v/release/debugger-bs/coreminer" alt="Release"/>
    </a>
    <br/>
    <a href="https://rust-lang.org">
        <img src="https://img.shields.io/badge/language-Rust-blue.svg" alt="Rust"/>
    </a>
    <a href="https://crates.io/crates/coreminer">
        <img alt="Crates.io MSRV" src="https://img.shields.io/crates/msrv/coreminer">
        <img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/coreminer">
    </a>
    <a href="https://docs.rs/coreminer">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/coreminer">
    </a>
</div>

# coreminer

* [GitHub](https://github.com/debugger-bs/coreminer)
* [crates.io](https://crates.io/crates/coreminer)
* [Documentation on docs.rs](https://docs.rs/crate/coreminer/)

A powerful debugger written in Rust that provides low-level debugging capabilities for programs that may not want to be debugged. Coreminer gives you deep control over program execution with robust DWARF debug information parsing.

## Features

- **Execution Control**: Set breakpoints, step through code, continue execution
- **Memory & Register Access**: Read from and write to process memory and CPU registers
- **Variable Inspection**: Read and write application variables using DWARF debug symbols
- **Stack Unwinding**: Generate and analyze stack backtraces
- **Disassembly**: View disassembled code at specific addresses
- **Process Inspection**: View process maps and executable layouts

## Installation

### Additional system dependencies

Coreminer depends on `libunwind-dev`. On Debian, it can be installed like
this. Other distributions provide similar packages.

```bash
apt install libunwind-dev
```

### From crates.io

```bash
cargo install coreminer
```

### From source

```bash
git clone https://github.com/debugger-bs/coreminer.git
cd coreminer
cargo build --release
```

The binary will be available at `./target/release/cm`.

## Quick Start

### Launch Coreminer

You can launch Coreminer with an optional executable path as a default target:

```bash
# Launch Coreminer without a target
cm

# Launch Coreminer with a default executable
cm ./target/debug/dummy
```

## Command-Line Interface

Coreminer provides a simple CLI that allows you to interact with the debugger. The interface supports command history for ease of use.

Once in the Coreminer CLI, you can use:

```
# Run a program with arguments
run ./target/debug/dummy arg1 arg2

# If launched with a default executable, you can run it with:
run

# Set a breakpoint at a specific address (hex)
bp 0x0000563087528176

# Continue execution
c

# Step over instructions
s
step

# View disassembly at some address, 20 bytes
d 0x0000563087528176 20

# View disassembly at some address, 20 bytes, showing the code exactly how it is
# in memory - including the int3 instructions set by breakpoints
d 0x0000563087528176 20 --literal

# Read registers
regs get

# View process memory
rmem 0x7fffffffe000

# Backtrace the call stack
bt

# Read a variable by name (requires debug information)
var my_variable_name

# Inspect a debug symbol (requires debug information)
sym main
sym i
```

### Advanced Stepping

```
# Step into a function call
si

# Step over a function call
su
sov

# Step out of the current function
so
```

### Memory and Register Manipulation

```
# Write a value to memory
wmem 0x7fffffffe000 0x42

# Set register value
regs set rip 19
```

### Working with Variables

```
# View a variable by name
var counter

# Write to a variable
vars counter 42
```

### Automatic Stepping

```
# Set up automatic stepping for N steps
set stepper 10
```

### Help and Exit

```
# Show available commands
help

# Exit the debugger
q
quit
exit
```

## Use Cases

- **Reverse Engineering**: Analyze and understand program behavior
- **Debugging Stripped Binaries**: Debug programs with limited debugging information
- **Security Research**: Analyze potentially malicious code in a controlled environment
- **Low-level Debugging**: Step through compiled code precisely

## Architecture

Coreminer is built around several key components:

- **Debuggee Control**: Via Linux ptrace API
- **DWARF Debug Info**: For symbol resolution and variable information
- **Stack Unwinding**: Using libunwind
- **Disassembly**: Powered by iced-x86

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: Add some amazing feature'`)
4. Push to the branch (`git push origin feat/amazing-feature`)
5. Open a Pull Request

Note: this project makes use of [conventional git commits](https://www.conventionalcommits.org/en/v1.0.0/).

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgements

- Thanks to the [BugStalker](https://github.com/godzie44/BugStalker) project for inspiration and reference on DWARF and unwinding implementation.
- Thanks to the [Sy Brand – Writing a Linux Debugger](https://blog.tartanllama.xyz/writing-a-linux-debugger-setup/) for his blog on writing a debugger.
