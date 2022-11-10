#pragma once
#define DUCKDB_BUILD_LOADABLE_EXTENSION

#include "duckdb.hpp"

namespace duckdb {
    class ScalarFunctionBuilder {
    public:
        explicit ScalarFunctionBuilder(std::string &name, duckdb::LogicalTypeId return_type) : name(name), return_type(
                return_type) {}
        std::unique_ptr<ScalarFunction> build();
    private:
        std::string&name;
        duckdb::LogicalTypeId return_type;
    };

    class RustFunctionData : public FunctionData {};
}
