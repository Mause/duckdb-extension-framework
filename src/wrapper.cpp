#include "wrapper.h"
#include "wrapper.hpp"

namespace duckdb {
    static void function(duckdb::DataChunk &, duckdb::ExpressionState &, duckdb::Vector &) {
    }

    std::unique_ptr <duckdb::ScalarFunction> ScalarFunctionBuilder::build() {
        vector <LogicalType> args{duckdb::LogicalTypeId::VARCHAR};

        return std::unique_ptr<duckdb::ScalarFunction>(new duckdb::ScalarFunction(
                this->name,
                args,
                duckdb::LogicalType(this->return_type),
                &function
        ));
    }
}
