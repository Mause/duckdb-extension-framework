use std::ffi::c_void;

use crate::duckly::{
    duckdb_init_get_column_count, duckdb_init_get_column_index, duckdb_init_info,
    duckdb_init_set_init_data, idx_t,
};

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
}
