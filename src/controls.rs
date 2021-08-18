use pyo3::prelude::*;

pub fn init(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Click>()?;
    m.add_class::<Selector>()?;
    m.add_class::<Slider>()?;
    m.add_class::<Switch>()?;
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
pub struct Selector {
    tracer: rillrate::Selector,
}

#[pyclass]
pub struct Slider {
    tracer: rillrate::Slider,
}

#[pyclass]
pub struct Switch {
    tracer: rillrate::Switch,
}
