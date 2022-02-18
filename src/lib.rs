use chrono::{NaiveDateTime, NaiveTime, Datelike, Timelike};
use pyo3::prelude::*;
use pyo3::types::PyDateTime;

#[derive(Debug)]
struct TimeWrapper(NaiveTime);

impl<'source> FromPyObject<'source> for TimeWrapper {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        let hour = ob.getattr("hour")?.extract()?;

        let t = NaiveTime::from_hms(hour, 0, 0);
        Ok(TimeWrapper(t))
    }
}

struct DateTimeResult(Result<NaiveDateTime, String>);

impl IntoPy<PyObject> for DateTimeResult {
    fn into_py(self, py: Python) -> PyObject {
        match self.0 {
            Ok(t) => {
                let year = t.year() as i32;
                let month = t.month() as u8;
                let day = t.day() as u8;
                let hour = t.hour() as u8;
                let min = t.minute() as u8;
                let sec = t.second() as u8;
                PyDateTime::new(py, year, month, day, hour, min, sec, 0, None).unwrap().into_py(py)
            }
            Err(_) => {
                py.None()
            }
        }
    }
}

#[pyfunction]
fn parse(s: String) -> PyResult<DateTimeResult> {
    Ok(DateTimeResult(fuzzydate::parse(&s)))
}

#[pyfunction]
fn parse_with_default_time(s: String, t: &PyAny) -> PyResult<DateTimeResult> {
    let t: TimeWrapper = t.extract()?;
    Ok(DateTimeResult(fuzzydate::parse_with_default_time(&s, t.0)))
}

/// A Python module implemented in Rust.
#[pymodule]
fn fuzzydate(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(parse_with_default_time, m)?)?;
    Ok(())
}
