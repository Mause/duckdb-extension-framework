use crate::as_string;
use crate::duckly::{
    duckdb_function_get_bind_data, duckdb_function_get_extra_info, duckdb_function_get_init_data,
    duckdb_function_get_local_init_data, duckdb_function_info, duckdb_function_set_error,
};
#[allow(unused)]
use crate::table_functions::{BindInfo, InitInfo, TableFunction};
use std::os::raw::c_char;

/// An interface to store and retrieve data during the function execution stage
#[derive(Debug)]
pub struct FunctionInfo(duckdb_function_info);

impl FunctionInfo {
    /// Report that an error has occurred while executing the function.
    ///
    /// # Arguments
    ///  * `error`: The error message
    pub fn set_error(&self, error: &str) {
        unsafe {
            duckdb_function_set_error(self.0, as_string!(error));
        }
    }
    /// Gets the bind data set by [`BindInfo::set_bind_data`] during the bind.
    ///
    /// Note that the bind data should be considered as read-only.
    /// For tracking state, use the init data instead.
    ///
    /// # Arguments
    /// * `returns`: The bind data object
    pub fn get_bind_data<T>(&self) -> *mut T {
        unsafe { duckdb_function_get_bind_data(self.0).cast() }
    }
    /// Gets the init data set by [`InitInfo::set_init_data`] during the init.
    ///
    /// # Arguments
    /// * `returns`: The init data object
    pub fn get_init_data<T>(&self) -> *mut T {
        unsafe { duckdb_function_get_init_data(self.0).cast() }
    }
    /// Retrieves the extra info of the function as set in [`TableFunction::set_extra_info`]
    ///
    /// # Arguments
    /// * `returns`: The extra info
    pub fn get_extra_info<T>(&self) -> *mut T {
        unsafe { duckdb_function_get_extra_info(self.0).cast() }
    }
    /// Gets the thread-local init data set by [`InitInfo::set_init_data`] during the local_init.
    ///
    /// # Arguments
    /// * `returns`: The init data object
    pub fn get_local_init_data<T>(&self) -> *mut T {
        unsafe { duckdb_function_get_local_init_data(self.0).cast() }
    }
}

impl From<duckdb_function_info> for FunctionInfo {
    fn from(ptr: duckdb_function_info) -> Self {
        Self(ptr)
    }
}
