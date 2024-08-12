from __future__ import annotations
from dataclasses import dataclass
from timeit import timeit

from pyinstrument import Profiler


@dataclass(frozen=True, slots=True)
class Quantity:
    value: float
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

    # xval = numpy.asarray([42.0])
    # yval = numpy.asarray([10.0])

    # n = 10
    # xval = numpy.random.rand(n)
    # yval = numpy.random.rand(n)

    # x = xval * astropy.units.meter
    # y = yval * astropy.units.second

    x = Quantity(xval)
    y = Quantity(yval)

    # with Profiler(0.0001) as p:
    #     for _ in range(10000):
    #         x * y
    # # p.open_in_browser()
    # p.print()

    t1 = timeit("x / y", globals={"x": x, "y": y}, number=1000)
    t2 = timeit("x / y", globals={"x": xval, "y": yval}, number=1000)
    print(t1 / t2)


if __name__ == "__main__":
    main()
