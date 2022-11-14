/// A replacement scan is a way to pretend that a table exists in DuckDB
/// For example, you can do the following:
/// ```sql
/// SELECT * from "hello.csv"
/// ```
/// and DuckDB will realise that you're referring to a CSV file, and read that instead
use crate::{
    duckly::{
        duckdb_replacement_scan_add_parameter, duckdb_replacement_scan_info,
        duckdb_replacement_scan_set_error, duckdb_replacement_scan_set_function_name,
    },
    Value,
};
use std::ffi::CString;
pub struct ReplacementScanInfo(pub(crate) duckdb_replacement_scan_info);

impl ReplacementScanInfo {
    /// Sets the replacement function name to use. If this function is called in the replacement callback, the replacement scan is performed. If it is not called, the replacement callback is not performed.
    pub fn set_function_name(&mut self, function_name: &str) {
        unsafe {
            let function_name = CString::new(function_name).unwrap();
            duckdb_replacement_scan_set_function_name(self.0, function_name.as_ptr());
        }
    }
    /// Adds a parameter to the replacement scan function.
    pub fn add_parameter(&mut self, parameter: Value) {
        unsafe {
            duckdb_replacement_scan_add_parameter(self.0, parameter.0);
        }
    }
    /// Report that an error has occurred while executing the replacement scan.
    pub fn set_error(&mut self, error: &str) {
        unsafe {
            let error = CString::new(error).unwrap();
            duckdb_replacement_scan_set_error(self.0, error.as_ptr());
        }
    }
}

impl From<duckdb_replacement_scan_info> for ReplacementScanInfo {
    fn from(value: duckdb_replacement_scan_info) -> Self {
        Self(value)
    }
}
