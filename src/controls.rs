use pyo3::prelude::*;
use rill_protocol::flow::core::Activity;

pub fn init(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Click>()?;
    m.add_class::<Selector>()?;
    m.add_class::<Slider>()?;
    m.add_class::<Switch>()?;
    Ok(())
}

fn activity(py: Python<'_>, activity: Activity) -> PyResult<&'_ PyAny> {
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

fn action<A: IntoPy<PyObject>>(py: Python<'_>, action: Option<A>) -> PyResult<PyObject> {
    match action {
        None => Ok(py.None()),
        Some(value) => {
            let module = PyModule::import(py, "rillrate")?;
            let class = module.getattr("Action")?;
            let instance = class.call1((value,))?;
            Ok(instance.into())
        }
    }
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
            Python::with_gil(|py| {
                let activity = activity(py, envelope.activity)?;
                let action = action(py, envelope.action)?;
                callback.call1(py, (activity, action))
            })
            .map_err(|err| err.into())
            .map(drop)
        });
    }

    fn apply(&mut self) {
        self.tracer.apply();
    }
}

#[pyclass]
pub struct Selector {
    tracer: rillrate::Selector,
}

#[pymethods]
impl Selector {
    #[new]
    fn new(path: String, label: String, options: Vec<String>) -> Self {
        let tracer = rillrate::Selector::new(path, label, options);
        Self { tracer }
    }

    fn sync_callback(&mut self, callback: PyObject) {
        self.tracer.sync_callback(move |envelope| {
            Python::with_gil(|py| {
                let activity = activity(py, envelope.activity)?;
                let action = action(py, envelope.action)?;
                callback.call1(py, (activity, action))
            })
            .map_err(|err| err.into())
            .map(drop)
        });
    }

    fn apply(&mut self, value: Option<String>) {
        self.tracer.apply(value);
    }
}

#[pyclass]
pub struct Slider {
    tracer: rillrate::Slider,
}

#[pymethods]
impl Slider {
    #[new]
    fn new(path: String, label: String, min: f64, max: f64, step: f64) -> Self {
        let tracer = rillrate::Slider::new(path, label, min, max, step);
        Self { tracer }
    }

    fn sync_callback(&mut self, callback: PyObject) {
        self.tracer.sync_callback(move |envelope| {
            Python::with_gil(|py| {
                let activity = activity(py, envelope.activity)?;
                let action = action(py, envelope.action)?;
                callback.call1(py, (activity, action))
            })
            .map_err(|err| err.into())
            .map(drop)
        });
    }

    fn apply(&mut self, value: f64) {
        self.tracer.apply(value);
    }
}

#[pyclass]
pub struct Switch {
    tracer: rillrate::Switch,
}

#[pymethods]
impl Switch {
    #[new]
    fn new(path: String, label: String) -> Self {
        let tracer = rillrate::Switch::new(path, label);
        Self { tracer }
    }

    fn sync_callback(&mut self, callback: PyObject) {
        self.tracer.sync_callback(move |envelope| {
            Python::with_gil(|py| {
                let activity = activity(py, envelope.activity)?;
                let action = action(py, envelope.action)?;
                callback.call1(py, (activity, action))
            })
            .map_err(|err| err.into())
            .map(drop)
        });
    }

    fn apply(&mut self, value: bool) {
        self.tracer.apply(value);
    }
}
