from typing import Self

import numpy

class Quantity:
    """A wrapped value with unit."""

    def __init__(self, value: float) -> None: ...
    @property
    def value(self) -> float:
        """The wrapped value."""

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __mul__(self, other: Self) -> Self: ...

class ArrayQuantity:
    """A wrapped array value with unit."""

    def __init__(self, value: numpy.ndarray) -> None: ...
    @property
    def value(self) -> numpy.ndarray:
        """The wrapped array."""

    def __str__(self) -> str: ...
    def __array_ufunc__(
        self,
        ufunc,
        method: str,
        *inputs,
        # out=None,  # FIXME: can/should we support `out`?
        **kwargs,
    ): ...

    # XXX: There may well be other overloads here that we need to consider!
    def __add__(self, other: Self) -> Self: ...
    def __mul__(self, other: Self) -> Self: ...
