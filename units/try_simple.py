from __future__ import annotations

from dataclasses import dataclass
from timeit import timeit

import numpy


@dataclass(frozen=True, slots=True)
class Quantity:
    value: float | numpy.ndarray
    # FIXME: add unit

    def __mul__(self, other) -> Quantity:
        if isinstance(other, Quantity):
            return Quantity(self.value * other.value)
        elif isinstance(other, float | int):
            return Quantity(self.value * other)
        return NotImplemented

    def __truediv__(self, other) -> Quantity:
        if isinstance(other, Quantity):
            return Quantity(self.value / other.value)
        elif isinstance(other, float | int):
            return Quantity(self.value / other)
        return NotImplemented


def main():
    xval = 42.0
    yval = 10.0

    n = 10
    xarrval = numpy.random.rand(n)
    yarrval = numpy.random.rand(n)

    x = Quantity(xval)
    y = Quantity(yval)
    xarr = Quantity(xarrval)
    yarr = Quantity(yarrval)

    # with Profiler(0.0001) as p:
    #     for _ in range(10000):
    #         x * y
    # # p.open_in_browser()
    # p.print()

    n_samples = 10000
    t1 = timeit("x * y", globals={"x": x, "y": y}, number=n_samples)
    t2 = timeit("x * y", globals={"x": xval, "y": yval}, number=n_samples)
    print(t1 / t2)

    t1 = timeit("x * y", globals={"x": xarr, "y": yarr}, number=n_samples)
    t2 = timeit("x * y", globals={"x": xarrval, "y": yarrval}, number=n_samples)
    print(t1 / t2)


if __name__ == "__main__":
    main()
