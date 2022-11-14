/// A table function is a function that returns a queryable table
mod bind_info;
mod function_info;
mod init_info;
mod replacement_scan;
mod table_function;
#[cfg(test)]
mod test_integration;

pub use self::bind_info::BindInfo;
pub use self::function_info::FunctionInfo;
pub use self::init_info::InitInfo;
pub use self::replacement_scan::ReplacementScanInfo;
pub use self::table_function::TableFunction;
