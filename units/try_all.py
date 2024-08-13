from timeit import timeit

import astropy.units
import numpy
import pint
import unyt
from ostrich import ArrayQuantity, Quantity, Unit


def _overhead_ratio(x, y, xval, yval, *, n_samples=1000) -> float:
    t1 = timeit("x * y", globals={"x": x, "y": y}, number=n_samples)
    t2 = timeit("x * y", globals={"x": xval, "y": yval}, number=n_samples)
    return t1 / t2


def main():
    xval = 42.0
    yval = 10.0

    n = 10
    xarrval = numpy.linspace(0, 1, n)
    yarrval = numpy.linspace(0, 1, n)

    print()
    print("Unyt:")
    x = xval * unyt.meter
    y = yval * unyt.second
    xarr = xarrval * unyt.meter
    yarr = yarrval * unyt.second
    print(f"Scalar overhead: {_overhead_ratio(x, y, xval, yval):.2f}")
    print(f"Array overhead:  {_overhead_ratio(xarr, yarr, xarrval, yarrval):.2f}")

    print()
    print("Astropy:")
    x = xval * astropy.units.meter
    y = yval * astropy.units.second
    xarr = xarrval * astropy.units.meter
    yarr = yarrval * astropy.units.second
    print(f"Scalar overhead: {_overhead_ratio(x, y, xval, yval):.2f}")
    print(f"Array overhead:  {_overhead_ratio(xarr, yarr, xarrval, yarrval):.2f}")

    print()
    print("Pint:")
    ureg = pint.UnitRegistry()
    x = xval * ureg.meter
    y = yval * ureg.second
    xarr = xarrval * ureg.meter
    yarr = yarrval * ureg.second
    print(f"Scalar overhead: {_overhead_ratio(x, y, xval, yval):.2f}")
    print(f"Array overhead:  {_overhead_ratio(xarr, yarr, xarrval, yarrval):.2f}")

    print()
    print("Ostrich:")
    x = Quantity(xval, Unit.Meter)
    y = Quantity(yval, Unit.Second)
    xarr = ArrayQuantity(xarrval, Unit.Meter)
    yarr = ArrayQuantity(yarrval, Unit.Second)
    print(f"Scalar overhead: {_overhead_ratio(x, y, xval, yval):.2f}")
    print(f"Array overhead:  {_overhead_ratio(xarr, yarr, xarrval, yarrval):.2f}")


if __name__ == "__main__":
    main()
