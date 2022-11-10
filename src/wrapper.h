#define DUCKDB_BUILD_LOADABLE_EXTENSION
#include "duckdb.h"

DUCKDB_API duckdb_logical_type duckdb_create_union(int nmembers, const char** names, const duckdb_type* types);
