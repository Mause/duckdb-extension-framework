#include "wrapper.h"

duckdb_logical_type duckdb_create_union(int nmembers, const char** names, const duckdb_type* types) {
  std::vector<std::pair<std::string, duckdb::LogicalType>> members;
  for (idx_t i=0; i<nmembers; i++) {
    members.emplace_back(
      string(names[i]),
      *(duckdb::LogicalType*) types[i]
    );
  }
  
  duckdb::LogicalType* utype = new duckdb::LogicalType;
  *utype = LogicalType::UNION(members);
  return utype;
}
