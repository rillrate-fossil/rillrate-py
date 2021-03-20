use once_cell::sync::OnceCell;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rillrate::{Col, RillRate, Row};

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
    m.add_class::<Counter>()?;
    m.add_class::<Gauge>()?;
    m.add_class::<Pulse>()?;
    m.add_class::<Histogram>()?;
    m.add_class::<Logger>()?;
    m.add_class::<Dict>()?;
    m.add_class::<Table>()?;
    Ok(())
}

#[pyclass]
pub struct Counter {
    tracer: rillrate::Counter,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(path: String) -> Self {
        let tracer = rillrate::Counter::create(&path).unwrap();
        Self { tracer }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.tracer.is_active()
    }

    fn inc(&mut self, _py: Python, delta: f64) {
        self.tracer.inc(delta);
    }
}

#[pyclass]
pub struct Gauge {
    tracer: rillrate::Gauge,
}

#[pymethods]
impl Gauge {
    #[new]
    fn new(path: String, min: f64, max: f64) -> Self {
        let tracer = rillrate::Gauge::create(&path, min, max).unwrap();
        Self { tracer }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.tracer.is_active()
    }

    fn set(&mut self, _py: Python, delta: f64) {
        self.tracer.set(delta);
    }
}

#[pyclass]
pub struct Histogram {
    tracer: rillrate::Histogram,
}

#[pymethods]
impl Histogram {
    #[new]
    fn new(path: String, levels: Vec<f64>) -> Self {
        let tracer = rillrate::Histogram::create(&path, &levels).unwrap();
        Self { tracer }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.tracer.is_active()
    }

    fn add(&mut self, _py: Python, value: f64) {
        self.tracer.add(value);
    }
}

#[pyclass]
pub struct Pulse {
    tracer: rillrate::Pulse,
}

#[pymethods]
impl Pulse {
    #[new]
    fn new(path: String, depth: Option<u32>) -> Self {
        let tracer = rillrate::Pulse::create(&path, depth).unwrap();
        Self { tracer }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.tracer.is_active()
    }

    fn inc(&mut self, _py: Python, delta: f64) {
        self.tracer.inc(delta);
    }

    fn dec(&mut self, _py: Python, delta: f64) {
        self.tracer.dec(delta);
    }

    fn set(&mut self, _py: Python, delta: f64) {
        self.tracer.set(delta);
    }
}

#[pyclass]
pub struct Logger {
    tracer: rillrate::Logger,
}

#[pymethods]
impl Logger {
    #[new]
    fn new(path: String) -> Self {
        let tracer = rillrate::Logger::create(&path).unwrap();
        Self { tracer }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.tracer.is_active()
    }

    fn log(&mut self, _py: Python, msg: String) {
        self.tracer.log(msg);
    }
}

#[pyclass]
pub struct Dict {
    tracer: rillrate::Dict,
}

#[pymethods]
impl Dict {
    #[new]
    fn new(path: String) -> Self {
        let tracer = rillrate::Dict::create(&path).unwrap();
        Self { tracer }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.tracer.is_active()
    }

    fn set(&mut self, _py: Python, key: String, value: String) {
        self.tracer.set(key, value);
    }
}

#[pyclass]
pub struct Table {
    tracer: rillrate::Table,
}

#[pymethods]
impl Table {
    #[new]
    fn new(path: String, columns: Vec<(u64, String)>) -> Self {
        let columns = columns
            .into_iter()
            .map(|(col, title)| (Col(col), title))
            .collect();
        let tracer = rillrate::Table::create(&path, columns).unwrap();
        Self { tracer }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.tracer.is_active()
    }

    fn add_row(&mut self, row: u64) {
        self.tracer.add_row(Row(row));
    }

    fn del_row(&mut self, row: u64) {
        self.tracer.del_row(Row(row));
    }

    fn set_cell(&mut self, row: u64, col: u64, value: String) {
        self.tracer.set_cell(Row(row), Col(col), value, None);
    }
}
