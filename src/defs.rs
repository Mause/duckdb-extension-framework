#![allow(clippy::needless_lifetimes)]
#![allow(clippy::upper_case_acronyms)]

use autocxx::prelude::*;
use cxx::kind::Opaque;
use cxx::{type_id, ExternType};

pub struct Vector {}
unsafe impl ExternType for Vector {
    type Id = type_id!("duckdb::Vector");
    type Kind = Opaque;
}

include_cpp! {
    #include "wrapper.hpp"
    generate!("duckdb::ScalarFunctionBuilder")
    generate!("duckdb::DataChunk")
    generate!("duckdb::ExpressionState")
    generate!("duckdb::Expression")
    generate!("duckdb::PreservedError")
    generate!("duckdb::RustFunctionData")
    generate!("duckdb::FunctionData")
    generate!("duckdb::ScalarFunction")
    generate!("duckdb::LogicalTypeId")
    extern_cpp_type!("duckdb::Vector", crate::defs::Vector)
}
pub use self::ffi::duckdb::{
    ClientContext, DataChunk, Expression, ExpressionState, LogicalTypeId, PreservedError,
    RustFunctionData, ScalarFunction, ScalarFunctionBuilder,
};
