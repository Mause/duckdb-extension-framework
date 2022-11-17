#include "duckdb.hpp"
#include "wrapper.hpp"

#include <memory>

static duckdb::child_list_t<duckdb::LogicalType>
getVector(idx_t n_pairs, const char *const *names, duckdb_logical_type const *types) {
    duckdb::child_list_t<duckdb::LogicalType> members;
    for (idx_t i = 0; i < n_pairs; i++) {
        members.emplace_back(
                std::string(names[i]),
                *(duckdb::LogicalType *) types[i]
        );
    }
    return members;
}

extern "C" {

duckdb_logical_type duckdb_create_struct_type(idx_t n_pairs, const char **names, const duckdb_logical_type *types) {
    auto *stype = new duckdb::LogicalType;
    *stype = duckdb::LogicalType::STRUCT(getVector(n_pairs, names, types));
    return stype;
}

duckdb_logical_type duckdb_create_union(idx_t nmembers, const char **names, const duckdb_logical_type *types) {
    auto *utype = new duckdb::LogicalType;
    *utype = duckdb::LogicalType::UNION(getVector(nmembers, names, types));
    return utype;
}

}
