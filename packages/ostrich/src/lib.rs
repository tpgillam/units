use numpy::PyUntypedArray;
use pyo3::prelude::*;

#[pyclass(module = "ostrich")]
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

#[pyclass(module = "ostrich")]
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
