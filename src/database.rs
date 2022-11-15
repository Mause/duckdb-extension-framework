use crate::database::DatabaseOwnership::{Borrowed, Owned};
use crate::duckly::{
    duckdb_add_replacement_scan, duckdb_close, duckdb_connect, duckdb_connection, duckdb_database,
    duckdb_delete_callback_t, duckdb_open, duckdb_open_ext, duckdb_replacement_callback_t,
};
use crate::Connection;
use crate::{check, Config};
use std::error::Error;
use std::ffi::{c_void, CString};
use std::ptr::{addr_of, addr_of_mut, null_mut};

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

        let filename = CString::new(":memory:")?;
        unsafe {
            check!(duckdb_open(filename.as_ptr(), &mut db));
        }
        Ok(Self(Owned(db)))
    }

    pub fn new_with_config(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let mut db: duckdb_database = null_mut();
        let error = CString::new("")?;
        let filename = CString::new(":memory:")?;
        let mut out_error = error.as_ptr().cast_mut();
        unsafe {
            check!(duckdb_open_ext(
                filename.as_ptr(),
                &mut db,
                config.0,
                addr_of_mut!(out_error)
            ));
        }
        Ok(Self(Owned(db)))
    }

    /// Construct a [`Database`] instance from a pointer passed to an extensions `init` function
    pub fn from_cpp_duckdb(ptr: *mut c_void) -> Self {
        Self(Borrowed(Wrapper { instance: ptr }))
    }

    pub fn connect(&self) -> Result<Connection, Box<dyn Error>> {
        let mut connection: duckdb_connection = null_mut();

        let db = self.get_ptr();

        unsafe {
            check!(duckdb_connect(db, &mut connection));
        }

        Ok(Connection::from(connection))
    }

    fn get_ptr(&self) -> duckdb_database {
        match &self.0 {
            Borrowed(wrapper) => addr_of!(wrapper) as duckdb_database,
            Owned(ptr) => *ptr,
        }
    }

    /// Add a replacement scan definition to the specified database
    ///
    /// # Safety
    /// The `extra_data` arg should live as long as the database
    ///
    /// # Arguments
    /// * `replacement`: The replacement scan callback
    /// * `extra_data`: Extra data that is passed back into the specified callback
    /// * `delete_callback`: The delete callback to call on the extra data, if any
    pub unsafe fn add_replacement_scan(
        &self,
        replacement: duckdb_replacement_callback_t,
        extra_data: *mut c_void,
        delete_callback: duckdb_delete_callback_t,
    ) {
        duckdb_add_replacement_scan(self.get_ptr(), replacement, extra_data, delete_callback);
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
    use crate::{Config, Connection};
    use std::any::{Any, TypeId};
    use std::error::Error;
    use std::ptr::null_mut;

    #[test]
    fn test_database_creation() -> Result<(), Box<dyn Error>> {
        let db = Database::new()?;
        let conn = db.connect()?;

        drop(db);

        assert_eq!(conn.type_id(), TypeId::of::<Connection>());

        drop(conn);

        Ok(())
    }

    #[test]
    fn test_with_config() -> Result<(), Box<dyn Error>> {
        let config = Config::new()?;

        let db = Database::new_with_config(&config)?;

        assert_ne!(db.get_ptr(), null_mut());

        Ok(())
    }
}
