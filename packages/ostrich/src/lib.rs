use std::iter;

use numpy::PyUntypedArray;
use pyo3::{
    prelude::*,
    types::{PyDict, PyString, PyTuple},
};

#[pyclass(frozen, module = "ostrich")]
#[derive(Debug)]
struct Quantity {
    #[pyo3(get)]
    value: f64,
}

#[pymethods]
impl Quantity {
    #[new]
    fn new(value: f64) -> Self {
        Quantity { value }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Quantity({})", self.value))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    fn __mul__(&self, other: &Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

#[pyclass(frozen, module = "ostrich")]
struct ArrayQuantity {
    #[pyo3(get)]
    value: Py<PyUntypedArray>,
}

#[pymethods]
impl ArrayQuantity {
    #[new]
    fn new(py: Python, value: Bound<PyUntypedArray>) -> ArrayQuantity {
        let x = Py::clone_ref(&value.unbind(), py);
        ArrayQuantity { value: x }
    }

    fn __str__(&self, py: Python) -> PyResult<String> {
        let value_str = self.value.call_method0(py, "__str__")?;
        Ok(format!("ArrayQuantity({})", value_str))
    }

    fn __mul__(&self, py: Python, other: &Self) -> PyResult<Self> {
        let x_any = self.value.bind(py).mul(other.value.bind(py))?.unbind();
        let value: Py<PyUntypedArray> = x_any.extract(py)?;
        Ok(Self { value })
    }

    #[pyo3(signature = (ufunc, method, *inputs, **kwargs))]
    fn __array_ufunc__(
        &self,
        py: Python,
        ufunc: Bound<PyAny>,
        method: Bound<PyString>,
        inputs: &Bound<PyTuple>,        // TODO: ref?
        kwargs: Option<&Bound<PyDict>>, // TODO: ref?
    ) -> PyResult<Self> {
        // TODO: do stuff to determine units.

        // // TODO: this seems an inefficient way of constructing a new tuple. Surely there's a better
        // // way?
        // let args_to_prepend = [ufunc, method];
        // let args_vec: Vec<_> = args_to_prepend
        //     .iter()
        //     .chain(inputs.as_slice().into_iter())
        //     .collect();
        // let args = PyTuple::new_bound(py, args_vec.iter());

        // let x_any = self
        //     .value
        //     .bind(py)
        //     .call_method("__array_ufunc__", args, kwargs)?
        //     .unbind();

        let attr = ufunc.getattr(method)?;
        dbg!(&attr);
        dbg!(inputs);
        // Iterate over the inputs and unwrap any instances of `ArrayQuantity`, and leave
        // everything else untouched.
        let unwrapped_inputs_vec = inputs
            .iter()
            .map(|x| {
                if let Ok(array_quantity) = x.extract::<Bound<ArrayQuantity>>() {
                    // FIXME: check that this `clone` isn't actually copying any data. We need to
                    //   do an INCREF but that's about it.
                    let y = array_quantity.get().value.bind(py).clone().into_any();
                    y
                } else {
                    x
                }
            })
            .collect::<Vec<_>>();

        let unwrapped_inputs = PyTuple::new_bound(py, unwrapped_inputs_vec.iter());
        dbg!(&unwrapped_inputs);
        let x_any = attr.call(unwrapped_inputs, kwargs)?;
        dbg!(&x_any);
        dbg!(&x_any.get_type());
        let value: Py<PyUntypedArray> = x_any.unbind().extract(py)?;
        dbg!(&value);
        Ok(Self { value })

        // dbg!(ufunc);
        // dbg!(method);
        // dbg!(inputs);
        // dbg!(kwargs);
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn ostrich(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Quantity>()?;
    m.add_class::<ArrayQuantity>()?;

    Ok(())
}
