#include "wrapper.h"

duckdb_logical_type duckdb_create_union() {
  duckdb::LogicalType* utype = new duckdb::LogicalType;
  return utype;
}
