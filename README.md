```
 ____                             _   _            
/ ___|  ___ _ __ _ __   ___ _ __ | |_(_)_ __   ___ 
\___ \ / _ \ '__| '_ \ / _ \ '_ \| __| | '_ \ / _ \
 ___) |  __/ |  | |_) |  __/ | | | |_| | | | |  __/
|____/ \___|_|  | .__/ \___|_| |_|\__|_|_| |_|\___|
                |_|                                 
```

> Lightning-fast Python type checker written in Rust

[![Build Status](https://img.shields.io/github/actions/workflow/status/yourusername/serpentine/ci.yml?branch=main)](https://github.com/yourusername/serpentine/actions)
[![Crates.io](https://img.shields.io/crates/v/serpentine.svg)](https://crates.io/crates/serpentine)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## ✨ Features

- **⚡ Blazing Fast** - Written in Rust for maximum performance
- **🎯 Accurate** - Catches type errors with precise error messages
- **🔧 Simple** - Easy to integrate into your workflow
- **📦 Lightweight** - Minimal dependencies, quick to install
- **🐍 Python 3.11+** - Supports modern Python type hints

## 🚀 Quick Start

```bash
# Install via cargo
cargo install serpentine

# Check a Python file
serp script.py

# Check with verbose output
serp script.py --verbose

# Check with strict mode (all variables)
serp script.py --including-implicit
```

## 📖 Usage

### Basic Type Checking

```python
# example.py
x: int = 42
y = "hello"
x = "world"  # Error: Type mismatch!
```

```bash
$ serp example.py
❌ Type check failed for 'example.py'
Error: Type "Literal['world']" is not assignable to declared type "int"
  "Literal['world']" is not assignable to "int"
```

### Command Line Options

```bash
serp script.py                       # Check with default settings
serp script.py --verbose             # Show all typed variables
serp script.py --including-implicit  # Check all variables ("strict" mode)
serp --help                          # Show help message
```

## 🛠️ Installation

### From Source

```bash
git clone https://github.com/yourusername/serpentine
cd serpentine
cargo install --path .
```

### Requirements

- Rust 1.70+
- Python 3.8+ (for checking Python 3.8+ code)

## 🤝 Contributing

We love contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

```bash
# Run tests
cargo test

# Run benchmarks
cargo bench

# Check code style
cargo clippy
```

## 🎯 What Serpentine Checks

- **Variable type annotations** - Enforces explicit type declarations
- **Type inference** - Automatically infers types from assignments
- **Reassignment checking** - Catches incompatible type changes
- **Basic types** - int, float, str, bool, list, dict, tuple
- **Literal types** - Special handling for string and integer literals

## 📄 License

MIT © Brody Critchlow

---

<p align="center">
  Made with ❤️ and 🦀
</p>
