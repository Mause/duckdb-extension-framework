use crate::duckly::*;

pub enum DuckDBType {
    Boolean = DUCKDB_TYPE_DUCKDB_TYPE_BOOLEAN as isize,
    Tinyint = DUCKDB_TYPE_DUCKDB_TYPE_TINYINT as isize,
    Smallint = DUCKDB_TYPE_DUCKDB_TYPE_SMALLINT as isize,
    Integer = DUCKDB_TYPE_DUCKDB_TYPE_INTEGER as isize,
    Bigint = DUCKDB_TYPE_DUCKDB_TYPE_BIGINT as isize,
    Utinyint = DUCKDB_TYPE_DUCKDB_TYPE_UTINYINT as isize,
    Usmallint = DUCKDB_TYPE_DUCKDB_TYPE_USMALLINT as isize,
    Uinteger = DUCKDB_TYPE_DUCKDB_TYPE_UINTEGER as isize,
    Ubigint = DUCKDB_TYPE_DUCKDB_TYPE_UBIGINT as isize,
    Float = DUCKDB_TYPE_DUCKDB_TYPE_FLOAT as isize,
    Double = DUCKDB_TYPE_DUCKDB_TYPE_DOUBLE as isize,
    Timestamp = DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP as isize,
    Date = DUCKDB_TYPE_DUCKDB_TYPE_DATE as isize,
    Time = DUCKDB_TYPE_DUCKDB_TYPE_TIME as isize,
    Interval = DUCKDB_TYPE_DUCKDB_TYPE_INTERVAL as isize,
    Hugeint = DUCKDB_TYPE_DUCKDB_TYPE_HUGEINT as isize,
    Varchar = DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR as isize,
    Blob = DUCKDB_TYPE_DUCKDB_TYPE_BLOB as isize,
    Decimal = DUCKDB_TYPE_DUCKDB_TYPE_DECIMAL as isize,
    TimestampS = DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_S as isize,
    TimestampMs = DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_MS as isize,
    TimestampNs = DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_NS as isize,
    Enum = DUCKDB_TYPE_DUCKDB_TYPE_ENUM as isize,
    List = DUCKDB_TYPE_DUCKDB_TYPE_LIST as isize,
    Struct = DUCKDB_TYPE_DUCKDB_TYPE_STRUCT as isize,
    Map = DUCKDB_TYPE_DUCKDB_TYPE_MAP as isize,
    Uuid = DUCKDB_TYPE_DUCKDB_TYPE_UUID as isize,
    Json = DUCKDB_TYPE_DUCKDB_TYPE_JSON as isize,
}
