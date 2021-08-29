use pyo3::prelude::*;
use rillrate::prime;

pub fn init(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Alert>()?;
    Ok(())
}

#[pyclass]
pub struct Alert {
    tracer: prime::Alert,
}

#[pymethods]
impl Alert {
    #[new]
    fn new(path: String) -> Self {
        let opts = prime::AlertOpts {};
        let tracer = prime::Alert::new(path, opts);
        Self { tracer }
    }

    fn notify(&mut self, message: String) {
        self.tracer.notify(message);
    }
}
