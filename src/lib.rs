use once_cell::sync::OnceCell;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rillrate::RillRate;

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
pub struct Pulse {
    tracer: rillrate::Pulse,
}

#[pymethods]
impl Pulse {
    #[new]
    fn new(path: String) -> Self {
        let tracer = rillrate::Pulse::create(&path).unwrap();
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
