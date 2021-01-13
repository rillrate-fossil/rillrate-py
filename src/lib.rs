use once_cell::sync::OnceCell;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rillrate::rill::providers;
use rillrate::{Path, RillRate};

static RILLRATE: OnceCell<RillRate> = OnceCell::new();

fn py_err(err: impl ToString) -> PyErr {
    PyTypeError::new_err(err.to_string())
}

#[pyfunction]
fn install(_py: Python) -> PyResult<()> {
    let rillrate = RillRate::from_env("rillratepy").map_err(py_err)?;
    RILLRATE
        .set(rillrate)
        .map_err(|_| py_err("can't install RillRate shared object"))?;
    Ok(())
}

#[pymodule]
fn rillrate(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pyfunction!(install))?;
    m.add_class::<Logger>()?;
    m.add_class::<Counter>()?;
    m.add_class::<Gauge>()?;
    Ok(())
}

/*
// TODO: Also allow using arrays
fn make_path(entries: Vec<String>) -> Path {
    let entries: Vec<_> = entries.into_iter().map(EntryId::from).collect();
    Path::from(entries)
}
*/
fn make_path(s: String) -> Path {
    s.parse().expect("can't parse path")
}

#[pyclass]
pub struct Logger {
    provider: providers::LogProvider,
}

#[pymethods]
impl Logger {
    #[new]
    fn new(s: String) -> Self {
        let path = make_path(s);
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
pub struct Counter {
    provider: providers::CounterProvider,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(s: String) -> Self {
        let path = make_path(s);
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
pub struct Gauge {
    provider: providers::GaugeProvider,
}

#[pymethods]
impl Gauge {
    #[new]
    fn new(s: String) -> Self {
        let path = make_path(s);
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
