use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};
use rill_protocol::flow::core::Activity;
use rrpack_prime::live_control::click::state::ClickAction;
use rrpack_prime::live_control::selector::state::SelectorAction;
use rrpack_prime::live_control::slider::state::SliderAction;

pub fn init(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Click>()?;
    m.add_class::<Selector>()?;
    m.add_class::<Slider>()?;
    m.add_class::<Switch>()?;
    Ok(())
}

fn activity<'a>(py: Python<'a>, activity: Activity) -> PyResult<&'a PyAny> {
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
            Python::with_gil(|py| {
                let activity = activity(py, envelope.activity)?;
                let action = click(py, envelope.action)?;
                callback.call1(py, (activity, action))
            })
            .map_err(|err| err.into())
            .map(drop)
        });
    }

    fn clicked(&mut self) {
        self.tracer.clicked();
    }
}

fn click<'a>(py: Python<'a>, action: Option<ClickAction>) -> PyResult<PyObject> {
    match action {
        None => Ok(py.None()),
        Some(ClickAction) => {
            let module = PyModule::import(py, "rillrate")?;
            let class = module.getattr("Action")?;
            let instance = class.call1(PyTuple::empty(py))?;
            Ok(instance.into())
        }
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
                let action = selector(py, envelope.action)?;
                callback.call1(py, (activity, action))
            })
            .map_err(|err| err.into())
            .map(drop)
        });
    }

    fn select(&mut self, value: Option<String>) {
        self.tracer.select(value);
    }
}

fn selector<'a>(py: Python<'a>, action: Option<SelectorAction>) -> PyResult<PyObject> {
    match action {
        None => Ok(py.None()),
        Some(SelectorAction { new_selected }) => {
            let module = PyModule::import(py, "rillrate")?;
            let class = module.getattr("Action")?;
            let instance = class.call1((new_selected,))?;
            Ok(instance.into())
        }
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
                let action = slider(py, envelope.action)?;
                callback.call1(py, (activity, action))
            })
            .map_err(|err| err.into())
            .map(drop)
        });
    }

    fn set(&mut self, value: f64) {
        self.tracer.set(value);
    }
}

fn slider<'a>(py: Python<'a>, action: Option<SliderAction>) -> PyResult<PyObject> {
    match action {
        None => Ok(py.None()),
        Some(SliderAction { new_value }) => {
            let module = PyModule::import(py, "rillrate")?;
            let class = module.getattr("Action")?;
            let instance = class.call1((new_value,))?;
            Ok(instance.into())
        }
    }
}

#[pyclass]
pub struct Switch {
    tracer: rillrate::Switch,
}
