use std::sync::Arc;

use either::Either;
use polars_core::schema::SchemaRef;
use pyo3::PyObject;

use self::python_dsl::{PythonOptionsDsl, PythonScanSource};
use crate::prelude::*;

impl LazyFrame {
    pub fn scan_from_python_function(
        schema: Either<PyObject, SchemaRef>,
        scan_fn: PyObject,
        pyarrow: bool,
        validate_schema: bool,
        inner_lfs: Option<Vec<LazyFrame>>,
        custom_explain_name: Option<String>,
    ) -> Self {
        DslPlan::PythonScan {
            options: PythonOptionsDsl {
                // Should be a python function that returns a generator
                scan_fn: Some(scan_fn.into()),
                schema_fn: Some(SpecialEq::new(Arc::new(schema.map_left(|obj| obj.into())))),
                python_source: if pyarrow {
                    PythonScanSource::Pyarrow
                } else {
                    PythonScanSource::IOPlugin
                },
                validate_schema,
                inner_plans: inner_lfs
                    .map(|v| v.into_iter().map(|lf| lf.dsl).collect())
                    .map(SpecialEq::new),
                custom_explain_name,
            },
        }
        .into()
    }
}
