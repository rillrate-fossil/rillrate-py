mod controls;
mod flows;
mod utils;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

fn py_err(err: impl ToString) -> PyErr {
    PyTypeError::new_err(err.to_string())
}

#[pyfunction]
fn install(_py: Python) -> PyResult<()> {
    rillrate::install("rillrate-py").map_err(py_err)?;
    Ok(())
}

#[pyfunction]
fn uninstall(_py: Python) -> PyResult<()> {
    rillrate::uninstall().map_err(py_err)?;
    Ok(())
}

/// IMPORTANT!
/// Don't forget to add these classes
/// to `__init__.py`!
#[pymodule]
fn rillrate(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pyfunction!(install))?;
    m.add_wrapped(wrap_pyfunction!(uninstall))?;
    register_prime(py, m)?;
    Ok(())
}

fn register_prime(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "prime")?;

    controls::init(py, submodule)?;
    flows::init(py, submodule)?;

    parent_module.add_submodule(submodule)?;

    Ok(())
}
