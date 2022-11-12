use crate::constants::LogicalTypeId;
use crate::database::Database;
use crate::duckly::{
    duckdb_bind_info, duckdb_data_chunk, duckdb_function_info, duckdb_init_info, duckdb_query,
};
use crate::duckly::{
    duckdb_destroy_result, duckdb_free, duckdb_result, duckdb_result_error,
    duckdb_state_DuckDBError, duckdb_value_varchar,
};
use crate::table_functions::{BindInfo, FunctionInfo, InitInfo, TableFunction};
use crate::{malloc_struct, DataChunk, LogicalType};
use std::error::Error;
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr::{null, null_mut};

#[repr(C)]
struct TestInitInfo {
    done: bool,
}

unsafe extern "C" fn func(info: duckdb_function_info, output: duckdb_data_chunk) {
    let info = FunctionInfo::from(info);
    let output = DataChunk::from(output);

    let init_info = info.get_init_data::<TestInitInfo>();

    if (*init_info).done {
        output.set_size(0);
    } else {
        (*init_info).done = true;

        let vector = output.get_vector::<&str>(0);

        let string = CString::new("hello world").expect("unable to build string");
        vector.assign_string_element(0, string.as_ptr());

        output.set_size(1);
    }
}

unsafe extern "C" fn init(info: duckdb_init_info) {
    let info = InitInfo::from(info);

    let data = malloc_struct::<TestInitInfo>();

    (*data).done = false;

    info.set_init_data(data.cast(), Some(duckdb_free))
}

unsafe extern "C" fn bind(info: duckdb_bind_info) {
    let info = BindInfo::from(info);

    info.add_result_column("column0", LogicalType::new(LogicalTypeId::Varchar));

    let param = info.get_parameter(0).get_varchar();

    assert_eq!("hello.json", param.to_str().unwrap());
}

#[test]
fn test_database_creation() -> Result<(), Box<dyn Error>> {
    let db = Database::new()?;
    let conn = db.connect()?;

    let table_function = TableFunction::default();
    table_function
        .add_parameter(&LogicalType::new(LogicalTypeId::Json))
        .set_name("read_json")
        .supports_pushdown(false)
        .set_function(Some(func))
        .set_init(Some(init))
        .set_bind(Some(bind));
    conn.register_table_function(table_function)?;

    let query = CString::new("select * from read_json('hello.json')")?;

    unsafe {
        let mut result: duckdb_result = mem::zeroed();
        let connection = conn.get_ptr();

        assert_ne!(connection, null_mut());

        if duckdb_query(connection, query.as_ptr(), &mut result) == duckdb_state_DuckDBError {
            let error = duckdb_result_error(&mut result);
            assert_ne!(error, null());
            let error = CStr::from_ptr(error);

            panic!("error: {}", error.to_str().unwrap());
        }

        let ptr = duckdb_value_varchar(&mut result, 0, 0);
        assert_ne!(ptr, null_mut());
        let value = CStr::from_ptr(ptr);

        assert_eq!(value.to_str()?, "hello world");

        duckdb_free(ptr.cast());

        duckdb_destroy_result(&mut result);
    };

    drop(conn);

    drop(db);

    Ok(())
}
