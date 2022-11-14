use crate::duckly::{
    duckdb_bind_add_result_column, duckdb_bind_get_extra_info, duckdb_bind_get_parameter,
    duckdb_bind_get_parameter_count, duckdb_bind_info, duckdb_bind_set_bind_data,
    duckdb_bind_set_cardinality, duckdb_bind_set_error, idx_t,
};
#[allow(unused)]
use crate::table_functions::TableFunction;
use crate::{as_string, LogicalType, Value};
use std::ffi::c_void;
use std::os::raw::c_char;

/// An interface to store and retrieve data during the function bind stage
#[derive(Debug)]
pub struct BindInfo {
    ptr: *mut c_void,
}

impl BindInfo {
    /// Adds a result column to the output of the table function.
    ///
    /// # Arguments
    ///  * `name`: The name of the column
    ///  * `type`: The logical type of the column
    pub fn add_result_column(&self, column_name: &str, column_type: LogicalType) {
        unsafe {
            duckdb_bind_add_result_column(self.ptr, as_string!(column_name), column_type.typ);
        }
    }
    /// Report that an error has occurred while calling bind.
    ///
    /// # Arguments
    ///  * `error`: The error message
    pub fn set_error(&self, error: &str) {
        unsafe {
            duckdb_bind_set_error(self.ptr, as_string!(error));
        }
    }
    /// Sets the user-provided bind data in the bind object. This object can be retrieved again during execution.
    ///
    /// # Arguments
    ///  * `extra_data`: The bind data object.
    ///  * `destroy`: The callback that will be called to destroy the bind data (if any)
    ///
    /// # Safety
    ///
    pub unsafe fn set_bind_data(
        &self,
        data: *mut c_void,
        free_function: Option<unsafe extern "C" fn(*mut c_void)>,
    ) {
        duckdb_bind_set_bind_data(self.ptr, data, free_function);
    }
    /// Retrieves the number of regular (non-named) parameters to the function.
    pub fn get_parameter_count(&self) -> u64 {
        unsafe { duckdb_bind_get_parameter_count(self.ptr) }
    }
    /// Retrieves the parameter at the given index.
    ///
    /// # Arguments
    ///  * `index`: The index of the parameter to get
    ///
    /// returns: The value of the parameter
    pub fn get_parameter(&self, param_index: u64) -> Value {
        unsafe { Value::from(duckdb_bind_get_parameter(self.ptr, param_index)) }
    }

    /// Sets the cardinality estimate for the table function, used for optimization.
    ///
    /// # Arguments
    /// * `cardinality`: The cardinality estimate
    /// * `is_exact`: Whether or not the cardinality estimate is exact, or an approximation
    pub fn set_cardinality(&self, cardinality: idx_t, is_exact: bool) {
        unsafe { duckdb_bind_set_cardinality(self.ptr, cardinality, is_exact) }
    }
    /// Retrieves the extra info of the function as set in [`TableFunction::set_extra_info`]
    ///
    /// # Arguments
    /// * `returns`: The extra info
    pub fn get_extra_info<T>(&self) -> *const T {
        unsafe { duckdb_bind_get_extra_info(self.ptr).cast() }
    }
}

impl From<duckdb_bind_info> for BindInfo {
    fn from(ptr: duckdb_bind_info) -> Self {
        Self { ptr }
    }
}
