use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rillrate::range::Range;
use rillrate::table::{Col, Row};

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

#[pymodule]
fn rillrate(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pyfunction!(install))?;
    m.add_wrapped(wrap_pyfunction!(uninstall))?;
    m.add_class::<Counter>()?;
    m.add_class::<Gauge>()?;
    m.add_class::<Pulse>()?;
    m.add_class::<Histogram>()?;
    m.add_class::<Board>()?;
    m.add_class::<Table>()?;
    //m.add_class::<Logger>()?;

    m.add_class::<Click>()?;

    Ok(())
}

#[pyclass]
pub struct Click {
    tracer: rillrate::Click,
}

#[pymethods]
impl Click {
    #[new]
    fn new(path: String, label: String) -> Self {
        let tracer = rillrate::Click::new(path, label);
        Self { tracer }
    }

    fn sync_callback(&mut self, callback: PyObject) {
        self.tracer.sync_callback(move |envelope| {
            // TODO: Pass envelope as parameters
            Python::with_gil(|py| callback.call(py, (), None))
                .map_err(|err| err.into())
                .map(drop)
        });
    }

    fn clicked(&mut self) {
        self.tracer.clicked();
    }
}

#[pyclass]
pub struct Counter {
    tracer: rillrate::Counter,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(path: String, realtime: bool) -> Self {
        let tracer = rillrate::Counter::new(path, realtime);
        Self { tracer }
    }

    fn inc(&mut self, delta: i64) {
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
        let spec = rillrate::gauge::GaugeSpec {
            pull_ms: None,
            range: Range::new(min, max),
        };
        let tracer = rillrate::Gauge::new(path, spec);
        Self { tracer }
    }

    fn set(&mut self, value: f64) {
        self.tracer.set(value);
    }
}

#[pyclass]
pub struct Pulse {
    tracer: rillrate::Pulse,
}

#[pymethods]
impl Pulse {
    #[new]
    fn new(path: String) -> Self {
        let spec = rillrate::pulse::PulseSpec::default();
        let tracer = rillrate::Pulse::new(path, spec);
        Self { tracer }
    }

    fn push(&mut self, value: f64) {
        self.tracer.push(value);
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
        let tracer = rillrate::Histogram::new(path, levels);
        Self { tracer }
    }

    fn add(&mut self, value: f64) {
        self.tracer.add(value);
    }
}

#[pyclass]
pub struct Board {
    tracer: rillrate::Board,
}

#[pymethods]
impl Board {
    #[new]
    fn new(path: String) -> Self {
        let tracer = rillrate::Board::new(path);
        Self { tracer }
    }

    fn set(&mut self, key: String, value: String) {
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
        let tracer = rillrate::Table::new(path, columns);
        Self { tracer }
    }

    fn add_row(&mut self, row: u64) {
        self.tracer.add_row(Row(row));
    }

    fn del_row(&mut self, row: u64) {
        self.tracer.del_row(Row(row));
    }

    fn set_cell(&mut self, row: u64, col: u64, value: String) {
        self.tracer.set_cell(Row(row), Col(col), value);
    }
}

/*
#[pyclass]
pub struct Logger {
    tracer: rillrate::Logger,
}

#[pymethods]
impl Logger {
    #[new]
    fn new(path: String) -> Self {
        let tracer = rillrate::Logger::new(&path).unwrap();
        Self { tracer }
    }

    fn is_active(&mut self) -> bool {
        self.tracer.is_active()
    }

    fn log(&mut self, msg: String) {
        self.tracer.log(msg);
    }
}
*/
