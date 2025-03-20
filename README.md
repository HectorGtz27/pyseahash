# pyseahash

## ✅ Prerequisites

- Python **3.13** installed
- Rust and Cargo installed
- `maturin` installed (`pip install maturin`)
- `pyseahash` module built and available as a `.whl` package

## 📦 Step 1: Clone `pyseahash`

First, clone the `pyseahash` repository from GitHub:

```powershell
git clone https://github.com/HectorGtz27/pyseahash.git
```

Navigate to the cloned repository:

```powershell
cd pyseahash
```

## 🛠 Step 2: Set Up the Virtual Environment

Check if the virtual environment `.venv` exists:

```powershell
ls
```

### ➤ If `.venv` does not exist, create it:

```powershell
py -m venv .venv
```

### ➤ Activate the virtual environment:

```powershell
.venv\Scripts\activate
```

## 🔧 Step 3: Install Dependencies

Ensure all required dependencies are installed inside `pyseahash`'s virtual environment.

### ➤ Install dependencies:

```powershell
pip install -r requirements.txt
```

_(If a `requirements.txt` file exists.)_

## 🏗️ Step 4: Create the `.whl` Package for `pyseahash` (IN CASE YOU DONT HAVE .whl)

If you need to build the `.whl` file for `pyseahash`, follow these steps:

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

After building, the `.whl` file will be located in:

```powershell
C:\Users\HectorGtz27\pyseahash\target\wheels\
```
