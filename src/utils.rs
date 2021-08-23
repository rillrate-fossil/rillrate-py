use pyo3::prelude::*;
use pyo3::types::PyDict;

pub fn get_from<'a, T>(kwargs: Option<&'a PyDict>, name: &'a str) -> PyResult<T>
where
    T: FromPyObject<'a> + Default,
{
    if let Some(dict) = kwargs {
        if let Some(value) = dict.get_item(name) {
            return value.extract();
        }
    }
    Ok(T::default())
}
