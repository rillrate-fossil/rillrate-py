use crate::utils::get_from;
use prime::table::{Col, Row};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rill_protocol::flow::core::FlowMode;
use rillrate::prime;

pub fn init(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Board>()?;
    m.add_class::<Counter>()?;
    m.add_class::<Gauge>()?;
    m.add_class::<Histogram>()?;
    m.add_class::<Pulse>()?;
    m.add_class::<LiveText>()?;
    m.add_class::<Table>()?;
    Ok(())
}

#[pyclass]
pub struct Counter {
    tracer: prime::Counter,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(path: String) -> Self {
        let opts = prime::CounterOpts {};
        let tracer = prime::Counter::new(path, FlowMode::Realtime, opts);
        Self { tracer }
    }

    fn inc(&mut self, delta: i64) {
        self.tracer.inc(delta);
    }
}

#[pyclass]
pub struct Gauge {
    tracer: prime::Gauge,
}

#[pymethods]
impl Gauge {
    #[new]
    #[args(kwargs = "**")]
    fn new(path: String, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let opts = prime::GaugeOpts {
            min: get_from(kwargs, "min")?,
            lower: get_from(kwargs, "lower")?,
            max: get_from(kwargs, "max")?,
            higher: get_from(kwargs, "higher")?,
        };
        let tracer = prime::Gauge::new(path, FlowMode::Realtime, opts);
        Ok(Self { tracer })
    }

    fn set(&mut self, value: f64) {
        self.tracer.set(value);
    }
}

#[pyclass]
pub struct Pulse {
    tracer: prime::Pulse,
}

#[pymethods]
impl Pulse {
    #[new]
    #[args(kwargs = "**")]
    fn new(path: String, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let opts = prime::PulseOpts {
            retain: get_from(kwargs, "retain")?,
            suffix: get_from(kwargs, "suffix")?,
            divisor: get_from(kwargs, "divisor")?,
            min: get_from(kwargs, "min")?,
            lower: get_from(kwargs, "lower")?,
            max: get_from(kwargs, "max")?,
            higher: get_from(kwargs, "higher")?,
        };
        let tracer = prime::Pulse::new(path, FlowMode::Realtime, opts);
        Ok(Self { tracer })
    }

    fn push(&mut self, value: f64) {
        self.tracer.push(value);
    }
}

#[pyclass]
pub struct Histogram {
    tracer: prime::Histogram,
}

#[pymethods]
impl Histogram {
    #[new]
    #[args(kwargs = "**")]
    fn new(path: String, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let opts = prime::HistogramOpts {
            levels: get_from(kwargs, "levels")?,
        };
        let tracer = prime::Histogram::new(path, FlowMode::Realtime, opts);
        Ok(Self { tracer })
    }

    fn add(&mut self, value: f64) {
        self.tracer.add(value);
    }
}

#[pyclass]
pub struct Board {
    tracer: prime::Board,
}

#[pymethods]
impl Board {
    #[new]
    fn new(path: String) -> Self {
        let opts = prime::BoardOpts {};
        let tracer = prime::Board::new(path, FlowMode::Realtime, opts);
        Self { tracer }
    }

    fn set(&mut self, key: String, value: String) {
        self.tracer.set(key, value);
    }

    fn remove(&mut self, key: String) {
        self.tracer.remove(key);
    }
}

#[pyclass]
pub struct LiveText {
    tracer: prime::LiveText,
}

#[pymethods]
impl LiveText {
    #[new]
    fn new(path: String) -> Self {
        let opts = prime::LiveTextOpts {};
        let tracer = prime::LiveText::new(path, FlowMode::Realtime, opts);
        Self { tracer }
    }

    fn set(&mut self, text: String) {
        self.tracer.set(text);
    }
}

#[pyclass]
pub struct Table {
    tracer: prime::Table,
}

#[pymethods]
impl Table {
    #[new]
    #[args(kwargs = "**")]
    fn new(path: String, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let opts = prime::TableOpts {
            columns: get_from(kwargs, "columns")?,
        };
        let tracer = prime::Table::new(path, FlowMode::Realtime, opts);
        Ok(Self { tracer })
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
    tracer: prime::Logger,
}

#[pymethods]
impl Logger {
    #[new]
    fn new(path: String) -> Self {
        let tracer = prime::Logger::new(&path).unwrap();
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
