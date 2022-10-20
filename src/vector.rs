use crate::{
    duckly::{
        duckdb_validity_row_is_valid, duckdb_validity_set_row_invalid,
        duckdb_validity_set_row_valid, duckdb_validity_set_row_validity, duckdb_vector,
        duckdb_vector_assign_string_element, duckdb_vector_assign_string_element_len,
        duckdb_vector_ensure_validity_writable, duckdb_vector_get_column_type,
        duckdb_vector_get_data, duckdb_vector_get_validity, duckdb_vector_size, idx_t,
    },
    LogicalType,
};
use std::ffi::{c_char, c_void};
use std::fmt::Debug;

pub struct Vector(duckdb_vector);

impl From<duckdb_vector> for Vector {
    fn from(ptr: duckdb_vector) -> Self {
        Self(ptr)
    }
}

impl Vector {
    pub fn get_data(&self) -> *mut c_void {
        unsafe { duckdb_vector_get_data(self.0) }
    }

    /// Assigns a string element in the vector at the specified location.
    ///
    /// # Arguments
    ///  * `index` - The row position in the vector to assign the string to
    ///  * `str` - The string
    ///  * `str_len` - The length of the string (in bytes)
    ///
    /// # Safety
    pub unsafe fn assign_string_element_len(
        &self,
        index: idx_t,
        str_: *const c_char,
        str_len: idx_t,
    ) {
        duckdb_vector_assign_string_element_len(self.0, index, str_, str_len);
    }

    /// Assigns a string element in the vector at the specified location.
    ///
    /// # Arguments
    ///  * `index` - The row position in the vector to assign the string to
    ///  * `str` - The null-terminated string"]
    ///
    /// # Safety
    pub unsafe fn assign_string_element(&self, index: idx_t, str_: *const c_char) {
        duckdb_vector_assign_string_element(self.0, index, str_);
    }
    pub fn get_column_type(&self) -> LogicalType {
        unsafe { LogicalType::from(duckdb_vector_get_column_type(self.0)) }
    }
    /// Retrieves the validity mask pointer of the specified vector.
    ///
    /// If all values are valid, this function MIGHT return NULL!
    ///
    /// The validity mask is a bitset that signifies null-ness within the data chunk. It is a series of uint64_t values, where each uint64_t value contains validity for 64 tuples. The bit is set to 1 if the value is valid (i.e. not NULL) or 0 if the value is invalid (i.e. NULL).
    ///
    /// Validity of a specific value can be obtained like this:
    ///
    /// idx_t entry_idx = row_idx / 64; idx_t idx_in_entry = row_idx % 64; bool is_valid = validity_maskentry_idx & (1 << idx_in_entry);
    ///
    /// Alternatively, the (slower) row_is_valid function can be used.
    ///
    /// returns: The pointer to the validity mask, or NULL if no validity mask is present
    pub fn get_validity(&self) -> ValidityMask {
        unsafe { ValidityMask(duckdb_vector_get_validity(self.0), duckdb_vector_size()) }
    }
    /// Ensures the validity mask is writable by allocating it.
    ///
    /// After this function is called, get_validity will ALWAYS return non-NULL. This allows null values to be written to the vector, regardless of whether a validity mask was present before.
    pub fn ensure_validity_writable(&self) {
        unsafe { duckdb_vector_ensure_validity_writable(self.0) };
    }
}

pub struct ValidityMask(*mut u64, idx_t);

impl Debug for ValidityMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = (0..self.1)
            .map(|row| if self.row_is_valid(row) { "." } else { "X" })
            .collect::<Vec<&str>>()
            .join("");

        f.debug_struct("ValidityMask")
            .field("validity", &base)
            .finish()
    }
}

impl ValidityMask {
    /// Returns whether or not a row is valid (i.e. not NULL) in the given validity mask.
    ///
    /// # Arguments
    ///  * `row`: The row index
    /// returns: true if the row is valid, false otherwise
    pub fn row_is_valid(&self, row: idx_t) -> bool {
        unsafe { duckdb_validity_row_is_valid(self.0, row) }
    }
    /// In a validity mask, sets a specific row to either valid or invalid.
    ///
    /// Note that ensure_validity_writable should be called before calling get_validity, to ensure that there is a validity mask to write to.
    ///
    /// # Arguments
    ///  * `row`: The row index
    ///  * `valid`: Whether or not to set the row to valid, or invalid
    pub fn set_row_validity(&self, row: idx_t, valid: bool) {
        unsafe { duckdb_validity_set_row_validity(self.0, row, valid) }
    }
    /// In a validity mask, sets a specific row to invalid.
    ///
    /// Equivalent to set_row_validity with valid set to false.
    ///
    /// # Arguments
    ///  * `row`: The row index
    pub fn set_row_invalid(&self, row: idx_t) {
        unsafe { duckdb_validity_set_row_invalid(self.0, row) }
    }
    /// In a validity mask, sets a specific row to valid.
    ///
    /// Equivalent to set_row_validity with valid set to true.
    ///
    /// # Arguments
    ///  * `row`: The row index
    pub fn set_row_valid(&self, row: idx_t) {
        unsafe { duckdb_validity_set_row_valid(self.0, row) }
    }
}
