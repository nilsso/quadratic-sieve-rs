Currently only a Python implementation is complete

Modules are messy, but split up.
Start in `python-version/qs-sieving.py`, and different
components (modular square roots, factoring, linear algebra)
are imported from other modules in `python-version/`.

Example terminal testing:
```bash
python3 python-version/qs-sieving.py 16843009 6 300
> (257, 65537)
```
