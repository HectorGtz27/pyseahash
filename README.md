# 🦀 pyseahash

A Python wrapper for **SeaHash**, implemented in Rust for high-performance hashing.

---

## ✅ Prerequisites

Before getting started, ensure you have the following installed:

- **Python 3.13**
- **Rust and Cargo** (Install from [rustup.rs](https://rustup.rs/))
- **maturin** (Install via `pip install maturin`)
- **pyseahash** built and available as a `.whl` package (if not, follow Step 4)

---

## 📦 Step 1: Clone `pyseahash`

First, clone the repository:

```powershell
git clone https://github.com/HectorGtz27/pyseahash.git
```

Navigate into the project directory:

```powershell
cd pyseahash
```

---

## 🛠 Step 2: Set Up a Virtual Environment

Check if the virtual environment `.venv` exists:

```powershell
ls
```

### ➤ If `.venv` does **not** exist, create it:

```powershell
py -m venv .venv
```

### ➤ Activate the virtual environment:

```powershell
.venv\Scripts\activate
```

---

## 🔧 Step 3: Install Dependencies

Ensure all required dependencies are installed inside the virtual environment.

### ➤ Install dependencies:

```powershell
pip install -r requirements.txt
```

_(Only if `requirements.txt` exists.)_

---

## 🏗️ Step 4: Build the `.whl` Package (If Not Available)

If you don't have the `.whl` package, follow these steps to build it:

### ➤ Navigate to the `pyseahash` directory:

```powershell
cd C:\Users\HectorGtz27\pyseahash
```

### ➤ Activate the virtual environment:

```powershell
.venv\Scripts\activate
```

### ➤ Build the `.whl` file using `maturin`:

```powershell
maturin build
```

After building, the `.whl` file will be available in:

```powershell
C:\Users\HectorGtz27\pyseahash\target\wheels\
```

---

### 🎯 Next Steps

- Install the `.whl` package via `pip install <path_to_whl_file>`
- Use `pyseahash` in your Python projects

---

### 📝 Notes

- Ensure Rust is installed and properly set up before building.
- If you encounter issues with `maturin build`, run `cargo check` inside the project directory to debug.
