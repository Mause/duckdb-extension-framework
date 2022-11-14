#[allow(unused)]
use crate::table_functions::{BindInfo, TableFunction};
use std::ffi::{c_void, CString};

use crate::duckly::{
    duckdb_init_get_bind_data, duckdb_init_get_column_count, duckdb_init_get_column_index,
    duckdb_init_get_extra_info, duckdb_init_info, duckdb_init_set_error, duckdb_init_set_init_data,
    duckdb_init_set_max_threads, idx_t,
};

/// An interface to store and retrieve data during the function init stage
#[derive(Debug)]
pub struct InitInfo(duckdb_init_info);

impl From<duckdb_init_info> for InitInfo {
    fn from(ptr: duckdb_init_info) -> Self {
        Self(ptr)
    }
}

impl InitInfo {
    /// # Safety
    pub unsafe fn set_init_data(
        &self,
        data: *mut c_void,
        freeer: Option<unsafe extern "C" fn(*mut c_void)>,
    ) {
        duckdb_init_set_init_data(self.0, data, freeer);
    }

    /// Returns the column indices of the projected columns at the specified positions.
    ///
    /// This function must be used if projection pushdown is enabled to figure out which columns to emit.
    ///
    /// returns: The column indices at which to get the projected column index
    pub fn get_column_indices(&self) -> Vec<idx_t> {
        let mut indices;
        unsafe {
            let column_count = duckdb_init_get_column_count(self.0);
            indices = Vec::with_capacity(column_count as usize);
            for i in 0..column_count {
                indices.push(duckdb_init_get_column_index(self.0, i))
            }
        }
        indices
    }

    /// Retrieves the extra info of the function as set in [`TableFunction::set_extra_info`]
    ///
    /// # Arguments
    /// * `returns`: The extra info
    pub fn get_extra_info<T>(&self) -> *const T {
        unsafe { duckdb_init_get_extra_info(self.0).cast() }
    }
    /// Gets the bind data set by [`BindInfo::set_bind_data`] during the bind.
    ///
    /// Note that the bind data should be considered as read-only.
    /// For tracking state, use the init data instead.
    ///
    /// # Arguments
    /// * `returns`: The bind data object
    pub fn get_bind_data<T>(&self) -> *const T {
        unsafe { duckdb_init_get_bind_data(self.0).cast() }
    }
    /// Sets how many threads can process this table function in parallel (default: 1)
    ///
    /// # Arguments
    /// * `max_threads`: The maximum amount of threads that can process this table function
    pub fn set_max_threads(&self, max_threads: idx_t) {
        unsafe { duckdb_init_set_max_threads(self.0, max_threads) }
    }
    /// Report that an error has occurred while calling init.
    ///
    /// # Arguments
    /// * `error`: The error message
    pub fn set_error(&self, error: CString) {
        unsafe { duckdb_init_set_error(self.0, error.as_ptr()) }
    }
}
