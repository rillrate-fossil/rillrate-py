use once_cell::sync::OnceCell;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use rillrate::rill::providers;
use rillrate::{EntryId, Path, RillRate};

static RILLRATE: OnceCell<RillRate> = OnceCell::new();

fn py_err(err: impl ToString) -> PyErr {
    PyTypeError::new_err(err.to_string())
}

#[pymodule]
fn rillrate(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "install")]
    fn install_py(_py: Python) -> PyResult<()> {
        let rillrate = RillRate::from_env("rillratepy").map_err(py_err)?;
        RILLRATE
            .set(rillrate)
            .map_err(|_| py_err("can't install RillRate shared object"))?;
        Ok(())
    }
    m.add_class::<LogProvider>()?;
    m.add_class::<CounterProvider>()?;
    m.add_class::<GaugeProvider>()?;
    Ok(())
}

// TODO: Parse it from "dotted.path"
fn make_path(entries: Vec<String>) -> Path {
    let entries: Vec<_> = entries.into_iter().map(EntryId::from).collect();
    Path::from(entries)
}

#[pyclass]
pub struct LogProvider {
    provider: providers::LogProvider,
}

#[pymethods]
impl LogProvider {
    #[new]
    fn new(entries: Vec<String>) -> Self {
        let path = make_path(entries);
        let provider = providers::LogProvider::new(path);
        Self { provider }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.provider.is_active()
    }

    fn log(&mut self, _py: Python, msg: String) {
        self.provider.log(msg, None);
    }
}

#[pyclass]
pub struct CounterProvider {
    provider: providers::CounterProvider,
}

#[pymethods]
impl CounterProvider {
    #[new]
    fn new(entries: Vec<String>) -> Self {
        let path = make_path(entries);
        let provider = providers::CounterProvider::new(path);
        Self { provider }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.provider.is_active()
    }

    fn inc(&mut self, _py: Python, delta: f64) {
        self.provider.inc(delta, None);
    }
}

#[pyclass]
pub struct GaugeProvider {
    provider: providers::GaugeProvider,
}

#[pymethods]
impl GaugeProvider {
    #[new]
    fn new(entries: Vec<String>) -> Self {
        let path = make_path(entries);
        let provider = providers::GaugeProvider::new(path);
        Self { provider }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.provider.is_active()
    }

    fn inc(&mut self, _py: Python, delta: f64) {
        self.provider.inc(delta, None);
    }

    fn dec(&mut self, _py: Python, delta: f64) {
        self.provider.dec(delta, None);
    }

    fn set(&mut self, _py: Python, delta: f64) {
        self.provider.set(delta, None);
    }
}
