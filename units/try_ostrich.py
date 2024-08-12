from timeit import timeit
import numpy
from ostrich import ArrayQuantity, Quantity
import gc


def main():
    xval = 42.0
    yval = 10.0

    n = 10
    xarrval = numpy.linspace(0, 1, n)
    yarrval = numpy.linspace(0, 1, n)


    x = Quantity(xval)
    y = Quantity(yval)
    xarr = ArrayQuantity(xarrval)
    yarr = ArrayQuantity(yarrval)
    # print(str(xarr))
    # print(xarr * yarr)

    # print(numpy.add.reduce(xarrval))
    # print(numpy.add.reduce(xarr))
    # print(xarr + yarr)

    n_samples = 10000

    t1 = timeit("x * y", globals={"x": x, "y": y}, number=n_samples)
    t2 = timeit("x * y", globals={"x": xval, "y": yval}, number=n_samples)
    print(t1 / t2)

    t1 = timeit("x * y", globals={"x": xarr, "y": yarr}, number=n_samples)
    t2 = timeit("x * y", globals={"x": xarrval, "y": yarrval}, number=n_samples)
    print(t1 / t2)


if __name__ == "__main__":
    main()
