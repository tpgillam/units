# Units

Experimenting with runtime unit systems in python, and benchmarking a few popular ones.

To fully build this you'll need rust installed.

Here's the output of running the `try_all.py` script, which runs some timings for simple operations.
The main point is that the `ostrich` library (a massively incomplete system prototyped within this repo) is basically the lowest overhead option that is possible in CPython:

```
moooo> make
moooo> uv run python units/try_all.py

Unyt:
Scalar overhead: 284.74
Array overhead:  19.93

Astropy:
Scalar overhead: 514.04
Array overhead:  27.25

Pint:
Scalar overhead: 326.73
Array overhead:  28.26

Ostrich:
Scalar overhead: 5.40
Array overhead:  1.68
```
