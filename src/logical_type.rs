use crate::constants::LogicalTypeId;
use crate::duckly::{
    duckdb_create_list_type, duckdb_create_logical_type, duckdb_create_map_type,
    duckdb_create_struct_type, duckdb_create_union, duckdb_destroy_logical_type,
    duckdb_get_type_id, duckdb_logical_type, idx_t,
};
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::ffi::{c_char, CString};
use std::ops::Deref;

/// Represents a logical type in the database - the underlying physical type can differ depending on the implementation
#[derive(Debug)]
pub struct LogicalType {
    pub(crate) typ: duckdb_logical_type,
}

impl LogicalType {
    pub fn new(typ: LogicalTypeId) -> Self {
        unsafe {
            Self {
                typ: duckdb_create_logical_type(typ as u32),
            }
        }
    }
    /// Creates a map type from its key type and value type.
    ///
    /// # Arguments
    /// * `type`: The key type and value type of map type to create.
    /// * `returns`: The logical type.
    pub fn new_map_type(key: &LogicalType, value: &LogicalType) -> Self {
        unsafe {
            Self {
                typ: duckdb_create_map_type(key.typ, value.typ),
            }
        }
    }
    /// Creates a list type from its child type.
    ///
    /// # Arguments
    /// * `type`: The child type of list type to create.
    /// * `returns`: The logical type.
    pub fn new_list_type(child_type: &LogicalType) -> Self {
        unsafe {
            Self {
                typ: duckdb_create_list_type(child_type.typ),
            }
        }
    }
    /// Make `LogicalType` for `struct`
    ///
    /// # Argument
    /// `shape` should be the fields and types in the `struct`
    pub fn new_struct_type(shape: HashMap<&str, LogicalType>) -> Self {
        Self::make_meta_type(shape, duckdb_create_struct_type)
    }
    /// Make `LogicalType` for `union`
    ///
    /// # Argument
    /// `shape` should be the variants in the `union`
    pub fn new_union_type(shape: HashMap<&str, LogicalType>) -> Self {
        Self::make_meta_type(shape, duckdb_create_union)
    }

    fn make_meta_type(
        shape: HashMap<&str, LogicalType>,
        x: unsafe extern "C" fn(
            nmembers: idx_t,
            names: *mut *const c_char,
            types: *const duckdb_logical_type,
        ) -> duckdb_logical_type,
    ) -> LogicalType {
        let keys: Vec<CString> = shape
            .keys()
            .map(|it| CString::new(it.deref()).unwrap())
            .collect();
        let values: Vec<duckdb_logical_type> = shape.values().map(|it| it.typ).collect();
        let name_ptrs = keys
            .iter()
            .map(|it| it.as_ptr())
            .collect::<Vec<*const c_char>>();

        unsafe {
            Self {
                typ: x(
                    shape.len().try_into().unwrap(),
                    name_ptrs.as_slice().as_ptr().cast_mut(),
                    values.as_slice().as_ptr(),
                ),
            }
        }
    }

    /// Retrieves the type class of a `duckdb_logical_type`.
    ///
    /// # Arguments
    /// * `returns`: The type id
    pub fn type_id(&self) -> LogicalTypeId {
        let id = unsafe { duckdb_get_type_id(self.typ) };

        FromPrimitive::from_u32(id).unwrap()
    }
}
impl Clone for LogicalType {
    fn clone(&self) -> Self {
        let type_id = self.type_id();

        Self::new(type_id)
    }
}

impl From<duckdb_logical_type> for LogicalType {
    fn from(ptr: duckdb_logical_type) -> Self {
        Self { typ: ptr }
    }
}

impl Drop for LogicalType {
    fn drop(&mut self) {
        unsafe {
            duckdb_destroy_logical_type(&mut self.typ);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::constants::LogicalTypeId;
    use crate::LogicalType;
    use std::collections::HashMap;
    #[test]
    fn test_logi() {
        let key = LogicalType::new(LogicalTypeId::Varchar);

        let value = LogicalType::new(LogicalTypeId::Utinyint);

        let map = LogicalType::new_map_type(&key, &value);

        assert_eq!(map.type_id(), LogicalTypeId::Map);

        let union_ = LogicalType::new_union_type(HashMap::from([
            ("number", LogicalType::new(LogicalTypeId::Bigint)),
            ("string", LogicalType::new(LogicalTypeId::Varchar)),
        ]));
        assert_eq!(union_.type_id(), LogicalTypeId::Union);

        let struct_ = LogicalType::new_struct_type(HashMap::from([
            ("number", LogicalType::new(LogicalTypeId::Bigint)),
            ("string", LogicalType::new(LogicalTypeId::Varchar)),
        ]));
        assert_eq!(struct_.type_id(), LogicalTypeId::Struct);
    }
}
