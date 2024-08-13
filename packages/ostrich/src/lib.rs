use numpy::PyUntypedArray;
use pyo3::{
    prelude::*,
    types::{PyDict, PyString, PyTuple},
};
use runtime_units::{
    units::{LengthUnit, TimeUnit},
    units_base::UnitDefinition,
};

// FIXME: can we avoid copy-pasting every single unit?
#[pyclass(frozen, module = "ostrich")]
enum Unit {
    Meter,
    Second,
}

impl Unit {
    // TODO: should this be `into`? Some kind of conversion trait seems like the rust-like way to
    // go here.
    fn to_unit_definition(&self) -> UnitDefinition {
        match self {
            Self::Meter => LengthUnit::meter.into(),
            Self::Second => TimeUnit::second.into(),
        }
    }

    // TODO: hmm not clear what to do here
    // fn from_unit_definition(unit_definition: &UnitDefinition) -> Self {
    //     match unit_definition {
    //         LengthUnit::meter => Self::Meter,
    //         TimeUnit::second => Self::Second,
    //     }
    // }
}

#[pyclass(frozen, module = "ostrich")]
#[derive(Debug)]
struct Quantity {
    #[pyo3(get)]
    value: f64,
    unit: UnitDefinition,
}

// TODO: how to define a getter to return us a unit?

#[pymethods]
impl Quantity {
    #[new]
    fn new(value: f64, unit: &Unit) -> Self {
        Quantity {
            value,
            unit: unit.to_unit_definition(),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "Quantity({}, {})",
            self.value,
            self.unit.unit_string()
        ))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    fn __mul__(&self, other: &Self) -> Self {
        Self {
            value: self.value * other.value,
            unit: self.unit * other.unit,
        }
    }
}

// FIXME: can we guarantee that this will indeed be frozen? And does it matter if the array is
// mutable? Need to work out what exact semantics "frozen" requires.
// NOTE: we're performing some gymnastics here to make this 'inherit' from NDArrayOperatorsMixin.
//  It's not possible to implement it entirely at this point; we have to first say that we inherit
//  from _some_ class, and then we insert the correct class into the MRO when building the module.
#[pyclass(frozen, module = "ostrich")]
struct ArrayQuantity {
    #[pyo3(get)]
    value: Py<PyUntypedArray>,
    unit: UnitDefinition,
}

#[pymethods]
impl ArrayQuantity {
    #[new]
    fn new(py: Python, value: Bound<PyUntypedArray>, unit: &Unit) -> Self {
        let x = Py::clone_ref(&value.unbind(), py);
        ArrayQuantity {
            value: x,
            unit: unit.to_unit_definition(),
        }
    }

    fn __str__(&self, py: Python) -> PyResult<String> {
        let value_str = self.value.call_method0(py, "__str__")?;
        Ok(format!(
            "ArrayQuantity({}, {})",
            value_str,
            self.unit.unit_string()
        ))
    }

    // TODO: unclear which of the arguments should be refs? Does it matter?
    #[pyo3(signature = (ufunc, method, *inputs, **kwargs))]
    fn __array_ufunc__<'a>(
        &self,
        ufunc: Bound<'a, PyAny>,
        method: Bound<'a, PyString>,
        inputs: &Bound<'a, PyTuple>,
        kwargs: Option<&Bound<'a, PyDict>>,
    ) -> PyResult<Bound<'a, PyAny>> {
        // FIXME: figure out what to do with units based on the operation.

        // Access the GIL token stored on one of the arguments.
        let py = ufunc.py();

        let attr = ufunc.getattr(method)?;

        // Iterate over the inputs and unwrap any instances of `ArrayQuantity`, and leave
        // everything else untouched.
        let unwrapped_inputs_vec = inputs
            .iter()
            .map(|x| {
                if let Ok(array_quantity) = x.extract::<Bound<ArrayQuantity>>() {
                    // NOTE: the `clone` here isn't copying any data. Rather, we've just got
                    // ourselves a `&Bound<PyAny>` but we need a `Bound<PyAny>` which we can get by
                    // calling `clone`. Under the hood this is equivalent to INCREF.
                    array_quantity.get().value.bind(py).as_any().clone()
                } else {
                    x
                }
            })
            .collect::<Vec<_>>();

        let unwrapped_inputs = PyTuple::new_bound(py, unwrapped_inputs_vec.iter());
        // dbg!(&unwrapped_inputs);
        let x_any = attr.call(unwrapped_inputs, kwargs)?;

        // Depending on the operation that we have performed, we might have a scalar at this point
        // (e.g. if calling `numpy.sum.reduce` on our array quantity).
        // PERF: performing the import and attribute access every time seems like a bad idea.
        let numpy_is_scalar = py.import_bound("numpy")?.getattr("isscalar")?;
        let is_scalar = numpy_is_scalar.call1((&x_any,))?.extract::<bool>()?;

        // dbg!(&x_any);
        // dbg!(&x_any.get_type());
        // dbg!(&is_scalar);

        if is_scalar {
            // FIXME: what to do here? We could try to return a Quantity; which means we have a
            //  non-deterministic return type.
            panic!("Oh scalar poo")
        } else {
            let value: Py<PyUntypedArray> = x_any.unbind().extract(py)?;
            // dbg!(&value);
            // NOTE: conceptually we just want to return a new `ArrayQuantity`, but since we're
            //  explicitly returning a `Bound<PyAny>` (rather than something that pyo3 will convert
            //  into a python object for us), we need to wrap it in a GIL-bound reference, and THEN
            //  convert it to a `PyAny` reference explicitly.
            // FIXME: unit is wrong!
            // FIXME: unit is wrong!
            Ok(Bound::new(
                py,
                ArrayQuantity {
                    value,
                    unit: self.unit,
                },
            )?
            .into_any())
        }
    }

    // TODO: tighten `other` to be an ArrayLike. And ensure that ArrayQuantity is array-like...
    // NOTE: something magical is going on here that requires us to name the first argument
    //  something other than `self` if we want to get a `&Bound<Self>` rather than just a `&Self`.
    //  This is probably because `self` is necessarily the receiver of a method. But the
    //  #[pymethods] macro can also deal with calling functions like `ArrayQuantity::__mul__`.
    fn __add__<'a>(slf: &Bound<'a, Self>, other: &Bound<'a, PyAny>) -> PyResult<Bound<'a, PyAny>> {
        // TODO: can we efficently factor out importing the correct ufunc? Can't be great to call
        //  `import_bound` every time we need to get the module.
        let py = other.py();
        let umath = py
            .import_bound("numpy")?
            .getattr("_core")?
            .getattr("umath")?;

        umath.getattr("add")?.call1((slf, other))
    }
    // PERF: this version of __mul__ is WAY faster than the one below that goes via the unfunc.
    //  My suspicion is that it's related to the numpy import.
    fn __mul__(&self, py: Python, other: &Self) -> PyResult<Self> {
        let bound_any = self
            .value
            .bind(py)
            .mul(other.value.bind(py))
            .expect("Multiplying arrays should work");
        let bound_array: Bound<PyUntypedArray> =
            bound_any.extract().expect("Result should be an array");
        Ok(Self {
            value: bound_array.unbind(),
            unit: self.unit * other.unit,
        })
    }
    // fn __mul__<'a>(slf: &Bound<'a, Self>, other: &Bound<'a, PyAny>) -> PyResult<Bound<'a, PyAny>> {
    //     let py = other.py();
    //     let umath = py
    //         .import_bound("numpy")?
    //         .getattr("_core")?
    //         .getattr("umath")?;

    //     umath.getattr("multiply")?.call1((slf, other))
    // }
}

/// True iff __array_ufunc__ exists on obj and is set to None.
fn _disables_array_ufunc(obj: &Bound<PyAny>) -> bool {
    match obj.getattr("__array_ufunc__") {
        Ok(array_ufunc) => array_ufunc.is_none(),
        Err(_) => false,
    }
}

// TODO: this is the NDArrayOperator mixin stuff, pasted from:
//  https://github.com/numpy/numpy/blob/v2.0.0/numpy/lib/mixins.py#L61-L183
//  Inheriting is a pain so better to just re-implement!

// # comparisons don't have reflected and in-place versions
// __lt__ = _binary_method(um.less, 'lt')
// __le__ = _binary_method(um.less_equal, 'le')
// __eq__ = _binary_method(um.equal, 'eq')
// __ne__ = _binary_method(um.not_equal, 'ne')
// __gt__ = _binary_method(um.greater, 'gt')
// __ge__ = _binary_method(um.greater_equal, 'ge')

// # numeric methods
// __add__, __radd__, __iadd__ = _numeric_methods(um.add, 'add')
// __sub__, __rsub__, __isub__ = _numeric_methods(um.subtract, 'sub')
// __mul__, __rmul__, __imul__ = _numeric_methods(um.multiply, 'mul')
// __matmul__, __rmatmul__, __imatmul__ = _numeric_methods(
//     um.matmul, 'matmul')
// # Python 3 does not use __div__, __rdiv__, or __idiv__
// __truediv__, __rtruediv__, __itruediv__ = _numeric_methods(
//     um.true_divide, 'truediv')
// __floordiv__, __rfloordiv__, __ifloordiv__ = _numeric_methods(
//     um.floor_divide, 'floordiv')
// __mod__, __rmod__, __imod__ = _numeric_methods(um.remainder, 'mod')
// __divmod__ = _binary_method(um.divmod, 'divmod')
// __rdivmod__ = _reflected_binary_method(um.divmod, 'divmod')
// # __idivmod__ does not exist
// # TODO: handle the optional third argument for __pow__?
// __pow__, __rpow__, __ipow__ = _numeric_methods(um.power, 'pow')
// __lshift__, __rlshift__, __ilshift__ = _numeric_methods(
//     um.left_shift, 'lshift')
// __rshift__, __rrshift__, __irshift__ = _numeric_methods(
//     um.right_shift, 'rshift')
// __and__, __rand__, __iand__ = _numeric_methods(um.bitwise_and, 'and')
// __xor__, __rxor__, __ixor__ = _numeric_methods(um.bitwise_xor, 'xor')
// __or__, __ror__, __ior__ = _numeric_methods(um.bitwise_or, 'or')

// # unary methods
// __neg__ = _unary_method(um.negative, 'neg')
// __pos__ = _unary_method(um.positive, 'pos')
// __abs__ = _unary_method(um.absolute, 'abs')
// __invert__ = _unary_method(um.invert, 'invert')

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn ostrich(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Unit>()?;
    m.add_class::<Quantity>()?;
    m.add_class::<ArrayQuantity>()?;

    Ok(())
}
