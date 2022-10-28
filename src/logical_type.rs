use crate::constants::DuckDBType;
use crate::duckly::{
    duckdb_create_logical_type, duckdb_create_map_type, duckdb_destroy_logical_type,
    duckdb_get_type_id, duckdb_logical_type,
};
use num_traits::FromPrimitive;

#[derive(Debug)]
pub struct LogicalType {
    pub(crate) typ: duckdb_logical_type,
}

impl LogicalType {
    pub fn new(typ: DuckDBType) -> Self {
        unsafe {
            Self {
                typ: duckdb_create_logical_type(typ as u32),
            }
        }
    }
    pub fn new_map_type(key: &LogicalType, value: &LogicalType) -> Self {
        unsafe {
            Self {
                typ: duckdb_create_map_type(key.typ, value.typ),
            }
        }
    }
    pub fn type_id(&self) -> DuckDBType {
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
    use crate::constants::DuckDBType;
    use crate::LogicalType;
    #[test]
    fn test_logi() {
        let key = LogicalType::new(DuckDBType::Varchar);

        let value = LogicalType::new(DuckDBType::Utinyint);

        let map = LogicalType::new_map_type(&key, &value);

        assert_eq!(map.type_id(), DuckDBType::Map);
    }
}
