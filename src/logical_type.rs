use crate::constants::LogicalTypeId;
use crate::duckly::{
    duckdb_create_list_type, duckdb_create_logical_type, duckdb_create_map_type,
    duckdb_create_union, duckdb_destroy_logical_type, duckdb_get_type_id, duckdb_logical_type,
};
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::ffi::{c_char, CString};
use std::ops::Deref;

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
    pub fn new_struct_type(_names: &[&str], _types: &[&LogicalType]) -> Self {
        todo!()
    }
    pub fn new_union_type(shape: HashMap<&str, LogicalType>) -> Self {
        unsafe {
            let keys: Vec<CString> = shape
                .keys()
                .map(|it| CString::new(it.deref()).unwrap())
                .collect();
            let values: Vec<duckdb_logical_type> = shape.values().map(|it| it.typ).collect();
            Self {
                typ: duckdb_create_union(
                    shape.len().try_into().unwrap(),
                    keys.iter()
                        .map(|it| it.as_ptr())
                        .collect::<Vec<*const c_char>>()
                        .as_slice()
                        .as_ptr()
                        .cast_mut(),
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
    #[test]
    fn test_logi() {
        let key = LogicalType::new(LogicalTypeId::Varchar);

        let value = LogicalType::new(LogicalTypeId::Utinyint);

        let map = LogicalType::new_map_type(&key, &value);

        assert_eq!(map.type_id(), LogicalTypeId::Map);
    }
}
