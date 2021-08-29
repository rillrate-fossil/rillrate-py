mod control;
mod transparent;
mod visual;

use pyo3::prelude::*;

pub fn register(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "prime")?;

    control::init(py, submodule)?;
    transparent::init(py, submodule)?;
    visual::init(py, submodule)?;

    parent_module.add_submodule(submodule)?;

    Ok(())
}
