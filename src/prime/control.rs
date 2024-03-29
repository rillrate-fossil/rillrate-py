use crate::utils::get_from;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rill_protocol::flow::core::Activity;
use rillrate::prime;

pub fn init(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Click>()?;
    m.add_class::<Input>()?;
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
    tracer: prime::Click,
}

#[pymethods]
impl Click {
    #[new]
    #[args(kwargs = "**")]
    fn new(path: String, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let opts = prime::ClickOpts {
            label: get_from(kwargs, "label")?,
        };
        let tracer = prime::Click::new(path, opts);
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
pub struct Input {
    tracer: prime::Input,
}

#[pymethods]
impl Input {
    #[new]
    #[args(kwargs = "**")]
    fn new(path: String, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let opts = prime::InputOpts {
            label: get_from(kwargs, "label")?,
        };
        let tracer = prime::Input::new(path, opts);
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
}

#[pyclass]
pub struct Selector {
    tracer: prime::Selector,
}

#[pymethods]
impl Selector {
    #[new]
    #[args(kwargs = "**")]
    fn new(path: String, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let opts = prime::SelectorOpts {
            label: get_from(kwargs, "label")?,
            options: get_from(kwargs, "options")?,
        };
        let tracer = prime::Selector::new(path, opts);
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

    fn apply(&mut self, value: Option<String>) {
        self.tracer.apply(value);
    }
}

#[pyclass]
pub struct Slider {
    tracer: prime::Slider,
}

#[pymethods]
impl Slider {
    #[new]
    #[args(kwargs = "**")]
    fn new(path: String, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let opts = prime::SliderOpts {
            label: get_from(kwargs, "label")?,
            min: get_from(kwargs, "min")?,
            max: get_from(kwargs, "max")?,
            step: get_from(kwargs, "step")?,
        };
        let tracer = prime::Slider::new(path, opts);
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

    fn apply(&mut self, value: f64) {
        self.tracer.apply(value);
    }
}

#[pyclass]
pub struct Switch {
    tracer: prime::Switch,
}

#[pymethods]
impl Switch {
    #[new]
    #[args(kwargs = "**")]
    fn new(path: String, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let opts = prime::SwitchOpts {
            label: get_from(kwargs, "label")?,
        };
        let tracer = prime::Switch::new(path, opts);
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

    fn apply(&mut self, value: bool) {
        self.tracer.apply(value);
    }
}
