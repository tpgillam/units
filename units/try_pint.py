from timeit import timeit
import numpy
import pint
from pyinstrument import Profiler


def main():
    ureg = pint.UnitRegistry()

    xval = 42.0
    yval = 10.0

    n = 4
    xarrval = numpy.random.rand(n)
    yarrval = numpy.random.rand(n)

    x = xval * ureg.meter
    y = yval * ureg.second
    xarr = xarrval * ureg.meter
    yarr = yarrval * ureg.second

    # with Profiler(0.0001) as p:
    #     for _ in range(10000):
    #         x * y
    # # p.open_in_browser()
    # p.print()

    t1 = timeit("x / y", globals={"x": x, "y": y}, number=1000)
    t2 = timeit("x / y", globals={"x": xval, "y": yval}, number=1000)
    print(t1 / t2)

    t1 = timeit("x / y", globals={"x": xarr, "y": yarr}, number=1000)
    t2 = timeit("x / y", globals={"x": xarrval, "y": yarrval}, number=1000)
    print(t1 / t2)


if __name__ == "__main__":
    main()
