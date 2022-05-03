use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::errors::ValResult;
use crate::input::Input;

use super::{BuildValidator, Extra, ValidateEnum, Validator};

/// This might seem useless, but it's useful in DictValidator to avoid Option<Validator> a lot
#[derive(Debug, Clone)]
pub struct AnyValidator;

impl BuildValidator for AnyValidator {
    const EXPECTED_TYPE: &'static str = "any";

    fn build(_schema: &PyDict, _config: Option<&PyDict>) -> PyResult<ValidateEnum> {
        Ok(Self.into())
    }
}

impl Validator for AnyValidator {
    fn validate<'s, 'data>(
        &'s self,
        py: Python<'data>,
        input: &'data dyn Input,
        _extra: &Extra,
    ) -> ValResult<'data, PyObject> {
        Ok(input.to_py(py))
    }

    fn get_name(&self, _py: Python) -> String {
        Self::EXPECTED_TYPE.to_string()
    }
}