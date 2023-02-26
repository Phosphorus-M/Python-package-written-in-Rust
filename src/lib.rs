extern crate pyo3;

use pyo3::prelude::*;

#[pyfunction]
#[pyo3(name = "add_two_numbers")]
fn add_two_numbers(py: Python<'_>, a:PyObject, b: PyObject) -> PyResult<PyObject> {
    if let (Ok(int1), Ok(int2)) = (a.extract::<i64>(py), b.extract::<i64>(py)){
        return Ok((int1 + int2).to_object(py));
    }
    if let (Ok(float1), Ok(float2)) = (a.extract::<f64>(py), b.extract::<f64>(py)){
        return Ok((float1 + float2).to_object(py));
    }
    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Not supported"))
}

#[pymodule]
#[pyo3(name = "_add_two_numbers")]
fn init(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_two_numbers, m)?)?;
    Ok(())
}
