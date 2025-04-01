# 🦀 pyseahash

A Python wrapper for **SeaHash**, implemented in Rust for high-performance hashing.

## ✨ Features

- Fast hashing via native Rust code
- Easy Python integration using [maturin](https://github.com/PyO3/maturin)
- Lightweight and portable

---

## 📦 Installation

You can install `pyseahash` directly from the GitHub repo using pip:

```bash
pip install git+https://github.com/HectorGtz27/pyseahash.git@main
```

---

## 🛠 For Local Development

If you want to develop or contribute to `pyseahash`, follow the steps below.

### ✅ Prerequisites

- Python **3.13**
- [Rust & Cargo](https://rustup.rs/)
- `maturin`:
  Install it with:
  ```bash
  pip install maturin
  ```

---

### 📁 Step 1: Clone the repository

```bash
git clone https://github.com/HectorGtz27/pyseahash.git
cd pyseahash
```

---

### 🧪 Step 2: (Optional) Set up a virtual environment

```bash
python -m venv .venv
source .venv/bin/activate      # On Linux/macOS
.venv\Scripts\activate         # On Windows
```

---

### 🏗️ Step 3: Build the package

Use `maturin` to build the wheel:

```bash
maturin build
```

The built `.whl` file will be available in:

```
target/wheels/
```

---

### 📥 Step 4: Install locally

After building, you can install the wheel directly:

```bash
pip install target/wheels/pyseahash-*.whl
```

---

## 🔄 Updating `pyseahash` in another project

If you're working on a project that depends on `pyseahash`, just reference it in your `requirements.in` or `requirements.txt` like so:

```text
pyseahash @ git+https://github.com/HectorGtz27/pyseahash.git@main
```
