#define DUCKDB_BUILD_LOADABLE_EXTENSION
#include "duckdb.h"

DUCKDB_API duckdb_logical_type duckdb_create_union(idx_t nmembers, const char** names, const duckdb_logical_type* types);

DUCKDB_API duckdb_logical_type duckdb_create_struct_type(idx_t n_pairs, const char** names, const duckdb_logical_type* types);
