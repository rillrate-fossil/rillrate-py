use crate::utils::get_from;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rill_protocol::flow::core::Activity;
use rillrate as rr;

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
    tracer: rr::Click,
}

#[pymethods]
impl Click {
    #[new]
    #[args(kwargs = "**")]
    fn new(path: String, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let opts = rr::ClickOpts {
            label: get_from(kwargs, "label")?,
        };
        let tracer = rr::Click::new(path, opts);
        Ok(Self { tracer })
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
    tracer: rr::Selector,
}

#[pymethods]
impl Selector {
    #[new]
    fn new(path: String, label: String, options: Vec<String>) -> Self {
        let spec = rr::SelectorSpec { label, options };
        let tracer = rr::Selector::new(path, spec);
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
    tracer: rr::Slider,
}

#[pymethods]
impl Slider {
    #[new]
    fn new(path: String, label: String, min: f64, max: f64, step: f64) -> Self {
        let spec = rr::SliderSpec {
            label,
            min,
            max,
            step,
        };
        let tracer = rr::Slider::new(path, spec);
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
    tracer: rr::Switch,
}

#[pymethods]
impl Switch {
    #[new]
    fn new(path: String, label: String) -> Self {
        let spec = rr::SwitchSpec { label };
        let tracer = rr::Switch::new(path, spec);
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
