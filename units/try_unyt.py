from timeit import timeit
import unyt
import numpy
from pyinstrument import Profiler


def main():
    xval = 42.0
    yval = 10.0

    n = 10
    xarrval = numpy.random.rand(n)
    yarrval = numpy.random.rand(n)

    x = xval * unyt.meter
    y = yval * unyt.second

    xarr = xarrval * unyt.meter
    yarr = yarrval * unyt.second

    # with Profiler(0.0001) as p:
    #     for _ in range(10000):
    #         x * y
    # # p.open_in_browser()
    # p.print()

    n_samples = 1000
    t1 = timeit("x * y", globals={"x": x, "y": y}, number=n_samples)
    t2 = timeit("x * y", globals={"x": xval, "y": yval}, number=n_samples)
    print(t1 / t2)

    t1 = timeit("x * y", globals={"x": xarr, "y": yarr}, number=n_samples)
    t2 = timeit("x * y", globals={"x": xarrval, "y": yarrval}, number=n_samples)
    print(t1 / t2)


if __name__ == "__main__":
    main()
