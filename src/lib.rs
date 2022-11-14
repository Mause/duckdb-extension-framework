#![deny(rustdoc::bare_urls)]
#![warn(rustdoc::invalid_html_tags)]
#![warn(rustdoc::private_doc_tests)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(unused_unsafe)]

//! This crate facilitates development of DuckDB extensions using Rust

mod config;
mod connection;
mod constants;
mod data_chunk;
mod database;
pub mod duckly;
mod error;
mod logical_type;
pub mod table_functions;
mod value;
mod vector;

use std::mem::size_of;

pub use crate::config::{get_configs, Config, ConfigItem, ConfigList};
pub use crate::connection::Connection;
pub use crate::constants::LogicalTypeId;
pub use crate::data_chunk::DataChunk;
pub use crate::database::Database;
pub use crate::logical_type::LogicalType;
pub use crate::value::Value;
pub use crate::vector::Vector;

use crate::duckly::duckdb_malloc;

/// # Safety
/// This function is obviously unsafe
pub unsafe fn malloc_struct<T>() -> *mut T {
    duckdb_malloc(size_of::<T>()).cast::<T>()
}
