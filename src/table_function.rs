use crate::duckly::{
    duckdb_create_table_function, duckdb_destroy_table_function,
    duckdb_table_function_add_parameter, duckdb_table_function_set_bind,
    duckdb_table_function_set_function, duckdb_table_function_set_init,
    duckdb_table_function_set_name, duckdb_table_function_supports_projection_pushdown,
};
use crate::logical_type::LogicalType;
use std::ffi::{c_void, CString};

pub struct TableFunction {
    pub(crate) ptr: *mut c_void,
}

impl Drop for TableFunction {
    fn drop(&mut self) {
        unsafe {
            duckdb_destroy_table_function(&mut self.ptr);
        }
    }
}

impl TableFunction {
    /// Sets whether or not the given table function supports projection pushdown.
    ///
    /// If this is set to true, the system will provide a list of all required columns in the `init` stage through
    /// the `InitInfo::get_column_indices` method.
    /// If this is set to false (the default), the system will expect all columns to be projected.
    ///
    /// pushdown: True if the table function supports projection pushdown, false otherwise.
    pub fn supports_pushdown(&self, supports: bool) -> &Self {
        unsafe {
            duckdb_table_function_supports_projection_pushdown(self.ptr, supports);
        }
        self
    }

    pub fn add_parameter(&self, logical_type: &LogicalType) -> &Self {
        unsafe {
            duckdb_table_function_add_parameter(self.ptr, logical_type.typ);
        }
        self
    }

    pub fn set_function(
        &self,
        func: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    ) -> &Self {
        unsafe {
            duckdb_table_function_set_function(self.ptr, func);
        }
        self
    }

    pub fn set_init(&self, init_func: Option<unsafe extern "C" fn(*mut c_void)>) -> &Self {
        unsafe {
            duckdb_table_function_set_init(self.ptr, init_func);
        }
        self
    }

    pub fn set_bind(&self, bind_func: Option<unsafe extern "C" fn(*mut c_void)>) -> &Self {
        unsafe {
            duckdb_table_function_set_bind(self.ptr, bind_func);
        }
        self
    }

    pub fn new() -> Self {
        Self {
            ptr: unsafe { duckdb_create_table_function() },
        }
    }

    pub fn set_name(&self, name: &str) -> &TableFunction {
        unsafe {
            let string = CString::from_vec_unchecked(name.as_bytes().into());
            duckdb_table_function_set_name(self.ptr, string.as_ptr());
        }
        self
    }
}
impl Default for TableFunction {
    fn default() -> Self {
        Self::new()
    }
}
