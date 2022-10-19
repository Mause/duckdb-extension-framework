use crate::check;
use crate::database::DatabaseOwnership::{Borrowed, Owned};
use crate::duckly::{
    duckdb_close, duckdb_connect, duckdb_connection, duckdb_database, duckdb_open,
};
use crate::Connection;
use std::error::Error;
use std::ffi::{c_void, CString};
use std::ptr::{addr_of, null_mut};

/// Equivalent of [`DatabaseData`](https://github.com/duckdb/duckdb/blob/50951241de3d9c06fac5719dcb907eb21163dcab/src/include/duckdb/main/capi_internal.hpp#L27), wraps `duckdb::DuckDB`
#[repr(C)]
#[derive(Debug)]
struct Wrapper {
    instance: *const c_void,
}

#[derive(Debug)]
enum DatabaseOwnership {
    Owned(duckdb_database),
    Borrowed(Wrapper),
}

#[derive(Debug)]
pub struct Database(DatabaseOwnership);

impl Database {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut db: duckdb_database = null_mut();

        let filename = CString::new(":memory:").unwrap();
        unsafe {
            check!(duckdb_open(filename.as_ptr(), &mut db));
        }
        Ok(Self(Owned(db)))
    }

    pub fn from_cpp_duckdb(ptr: *mut c_void) -> Self {
        Self(Borrowed(Wrapper { instance: ptr }))
    }

    pub fn connect(&self) -> Result<Connection, Box<dyn Error>> {
        let mut connection: duckdb_connection = null_mut();

        let db = match &self.0 {
            Borrowed(wrapper) => addr_of!(wrapper) as duckdb_database,
            Owned(ptr) => *ptr,
        };

        unsafe {
            check!(duckdb_connect(db, &mut connection));
        }

        Ok(Connection::from(connection))
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if let Owned(mut ptr) = self.0 {
            unsafe { duckdb_close(&mut ptr) }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::database::Database;
    use crate::Connection;
    use std::any::{Any, TypeId};
    use std::error::Error;

    #[test]
    fn test_database_creation() -> Result<(), Box<dyn Error>> {
        let db = Database::new()?;
        let conn = db.connect()?;

        drop(db);

        assert_eq!(conn.type_id(), TypeId::of::<Connection>());

        drop(conn);

        Ok(())
    }
}
