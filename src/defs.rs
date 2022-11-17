#![allow(clippy::needless_lifetimes)]
#![allow(clippy::upper_case_acronyms)]

use autocxx::prelude::*;

include_cpp! {
    #include "wrapper.hpp"
    generate!("duckdb::LogicalTypeId")
}
pub use self::ffi::duckdb::LogicalTypeId;
