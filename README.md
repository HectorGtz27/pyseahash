## 🏗️ Create the `.whl` Package for `pyseahash`

If you need to build the `.whl` file for `pyseahash`, follow these steps:

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

## 🚀 Step 5: Run `dicom_retriever`

Once everything is set up, run the main script:

```powershell
python main.py
```

## 🔄 Updating `pyseahash`

If `pyseahash` is updated, rebuild it and reinstall:

```powershell
cd C:\Users\HectorGtz27\pyseahash
.venv\Scripts\activate
maturin build
pip install --upgrade C:\Users\HectorGtz27\pyseahash\target\wheels\pyseahash-0.1.0-cp313-cp313-win_amd64.whl
```

Then switch back to `dicom_retriever` and run it again:

```powershell
cd C:\Users\HectorGtz27\Documents\Hector\FNNDSC\dicom_retriever
.venv\Scripts\activate
python main.py
```

## 🎯 Summary

- Always **activate the virtual environment** before running the script.
- Ensure `pyseahash` is **installed inside `dicom_retriever`'s virtual environment`**.
- If `pyseahash` is updated, **rebuild & reinstall it**.
- Run `python main.py` inside `dicom_retriever` to execute the project.
