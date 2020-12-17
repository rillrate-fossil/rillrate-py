use pyo3::prelude::*;
use rill::prelude::{EntryId, LogProvider as RillLogProvider, Path};

#[pyclass]
pub struct LogProvider {
    provider: RillLogProvider,
}

#[pymethods]
impl LogProvider {
    #[new]
    fn new(entries: Vec<String>) -> Self {
        let entries: Vec<_> = entries.into_iter().map(EntryId::from).collect();
        let path = Path::from(entries);
        let provider = RillLogProvider::new(path);
        Self { provider }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.provider.is_active()
    }

    fn log(&mut self, _py: Python, msg: String) {
        self.provider.log(msg, None);
    }
}

#[pymodule]
fn rill(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // TODO: Return error here
    rill::install("python").unwrap();
    m.add_class::<LogProvider>()?;
    Ok(())
}
