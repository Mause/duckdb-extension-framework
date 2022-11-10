use crate::duckly::{
    duckdb_create_table_function, duckdb_delete_callback_t, duckdb_destroy_table_function,
    duckdb_table_function, duckdb_table_function_add_parameter, duckdb_table_function_init_t,
    duckdb_table_function_set_bind, duckdb_table_function_set_extra_info,
    duckdb_table_function_set_function, duckdb_table_function_set_init,
    duckdb_table_function_set_local_init, duckdb_table_function_set_name,
    duckdb_table_function_supports_projection_pushdown,
};
use crate::logical_type::LogicalType;
#[allow(unused)]
use crate::InitInfo;
use std::ffi::{c_void, CString};

/// A function that returns a queryable table
#[derive(Debug)]
pub struct TableFunction {
    pub(crate) ptr: duckdb_table_function,
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
    /// the [`InitInfo::get_column_indices`] method.
    /// If this is set to false (the default), the system will expect all columns to be projected.
    ///
    /// # Arguments
    ///  * `pushdown`: True if the table function supports projection pushdown, false otherwise.
    pub fn supports_pushdown(&self, supports: bool) -> &Self {
        unsafe {
            duckdb_table_function_supports_projection_pushdown(self.ptr, supports);
        }
        self
    }

    /// Adds a parameter to the table function.
    ///
    /// # Arguments
    ///  * `logical_type`: The type of the parameter to add.
    pub fn add_parameter(&self, logical_type: &LogicalType) -> &Self {
        unsafe {
            duckdb_table_function_add_parameter(self.ptr, logical_type.typ);
        }
        self
    }

    /// Sets the main function of the table function
    ///
    /// # Arguments
    ///  * `function`: The function
    pub fn set_function(
        &self,
        func: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    ) -> &Self {
        unsafe {
            duckdb_table_function_set_function(self.ptr, func);
        }
        self
    }

    /// Sets the init function of the table function
    ///
    /// # Arguments
    ///  * `function`: The init function
    pub fn set_init(&self, init_func: Option<unsafe extern "C" fn(*mut c_void)>) -> &Self {
        unsafe {
            duckdb_table_function_set_init(self.ptr, init_func);
        }
        self
    }

    /// Sets the bind function of the table function
    ///
    /// # Arguments
    ///  * `function`: The bind function
    pub fn set_bind(&self, bind_func: Option<unsafe extern "C" fn(*mut c_void)>) -> &Self {
        unsafe {
            duckdb_table_function_set_bind(self.ptr, bind_func);
        }
        self
    }

    /// Creates a new empty table function.
    pub fn new() -> Self {
        Self {
            ptr: unsafe { duckdb_create_table_function() },
        }
    }

    /// Sets the name of the given table function.
    ///
    /// # Arguments
    ///  * `name`: The name of the table function
    pub fn set_name(&self, name: &str) -> &TableFunction {
        unsafe {
            let string = CString::from_vec_unchecked(name.as_bytes().into());
            duckdb_table_function_set_name(self.ptr, string.as_ptr());
        }
        self
    }

    /// Assigns extra information to the table function that can be fetched during binding, etc.
    ///
    /// # Arguments
    /// * `extra_info`: The extra information
    /// * `destroy`: The callback that will be called to destroy the bind data (if any)
    ///
    /// # Safety
    pub unsafe fn set_extra_info(
        &self,
        extra_info: *mut c_void,
        destroy: duckdb_delete_callback_t,
    ) {
        duckdb_table_function_set_extra_info(self.ptr, extra_info, destroy);
    }

    /// Sets the thread-local init function of the table function
    ///
    /// # Arguments
    /// * `init`: The init function
    pub fn set_local_init(&self, init: duckdb_table_function_init_t) {
        unsafe { duckdb_table_function_set_local_init(self.ptr, init) };
    }
}
impl Default for TableFunction {
    fn default() -> Self {
        Self::new()
    }
}
