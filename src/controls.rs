use pyo3::prelude::*;
use pyo3::types::PyTuple;
use rill_protocol::flow::core::Activity;
use rrpack_prime::live_control::click::state::ClickAction;

pub fn init(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Click>()?;
    m.add_class::<Selector>()?;
    m.add_class::<Slider>()?;
    m.add_class::<Switch>()?;
    Ok(())
}

fn activity<'a>(py: Python<'a>, activity: &'a Activity) -> PyResult<&'a PyAny> {
    let attr = {
        match activity {
            Activity::Suspend => "SUSPEND",
            Activity::Awake => "AWAKE",
            Activity::Disconnected => "DISCONNECTED",
            Activity::Connected => "CONNECTED",
            Activity::Action => "ACTION",
        }
    };
    let module = PyModule::import(py, "rillrate")?;
    let activity = module.getattr("Activity")?;
    activity.getattr(attr)
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
            Python::with_gil(|py| {
                let activity = activity(py, &envelope.activity)?;
                let action = match envelope.action {
                    None => {
                        py.None()
                    }
                    Some(ClickAction) => {
                        PyTuple::empty(py).into()
                    }
                };
                callback.call(py, (activity, action), None)
            })
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
