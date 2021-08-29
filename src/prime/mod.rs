mod controls;
mod flows;

use pyo3::prelude::*;

pub fn register(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "prime")?;

    controls::init(py, submodule)?;
    flows::init(py, submodule)?;

    parent_module.add_submodule(submodule)?;

    Ok(())
}
