use crate::check;
use crate::duckly::{duckdb_connection, duckdb_disconnect, duckdb_register_table_function};
use crate::table_functions::TableFunction;

/// A connection to a database. This represents a (client) connection that can
/// be used to query the database.
#[derive(Debug)]
pub struct Connection {
    ptr: duckdb_connection,
}

impl From<duckdb_connection> for Connection {
    fn from(ptr: duckdb_connection) -> Self {
        Self { ptr }
    }
}

impl Connection {
    /// Register the table function object within the given connection.
    ///
    /// The function requires at least a name, a bind function, an init function and a main function.
    ///
    /// If the function is incomplete or a function with this name already exists DuckDBError is returned.
    ///
    /// # Arguments
    ///  * `function`: The function pointer
    /// returns: Whether or not the registration was successful.
    pub fn register_table_function(
        &self,
        table_function: TableFunction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            check!(duckdb_register_table_function(self.ptr, table_function.ptr));
        }
        Ok(())
    }

    /// Returns the internal connection pointer
    pub fn get_ptr(&self) -> duckdb_connection {
        self.ptr
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            duckdb_disconnect(&mut self.ptr);
        }
    }
}
