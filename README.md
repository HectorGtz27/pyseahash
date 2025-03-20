# pyseahash

A high-performance Python binding for the [seahash](https://github.com/ticki/seahash) algorithm, implemented in Rust.

## Description

`pyseahash` provides Python bindings for the seahash algorithm, which is a fast non-cryptographic hash function. The implementation is written in Rust and exposed to Python using PyO3, offering near-native performance.

## Features

- Fast, non-cryptographic hashing
- Zero-copy Python bindings
- Thread-safe implementation
- Compatible with Python 3.8+

## Installation

You can install `pyseahash` using pip:

```bash
pip install pyseahash
```

## Usage

```python
import pyseahash

# Hash a string
hash_value = pyseahash.hash("Hello, World!")

# Hash bytes
hash_value = pyseahash.hash(b"Hello, World!")

# Hash a file
with open("example.txt", "rb") as f:
    hash_value = pyseahash.hash(f.read())
```

## Development

This project uses:

- Rust for the core implementation
- PyO3 for Python bindings
- Maturin for building the Python package

To set up the development environment:

1. Install Rust and Python 3.8+
2. Install maturin:
   ```bash
   pip install maturin
   ```
3. Build the package:
   ```bash
   maturin develop
   ```
