#![allow(dead_code)]
#![allow(unused_variables)]

use crate::defs::{
    ClientContext, DataChunk, Expression, ExpressionState, PreservedError, RustFunctionData, Vector,
};
pub use crate::defs::{ScalarFunction, ScalarFunctionBuilder};
use cxx::UniquePtr;
use std::pin::Pin;

pub type ScalarFunctionT = fn(
    args: &DataChunk,
    state: &ExpressionState,
    result: Pin<&mut Vector>,
) -> UniquePtr<PreservedError>;
pub type BindFunctionT = fn(
    context: &ClientContext,
    bound_function: &ScalarFunction,
    arguments: &mut [UniquePtr<Expression>],
) -> UniquePtr<RustFunctionData>;

impl ScalarFunctionBuilder {
    pub fn set_function(self: Pin<&mut Self>, function: ScalarFunctionT) {
        //     unsafe { otherffi::set_function(self, function) };
    }
    pub fn set_bind(self: Pin<&mut Self>, function: BindFunctionT) {
        //     unsafe { otherffi::set_bind(self, function) };
    }
}

#[cfg(test)]
mod test {
    use crate::defs::{
        ClientContext, DataChunk, Expression, ExpressionState, LogicalTypeId, PreservedError,
        RustFunctionData, Vector,
    };
    use crate::scalar_function::{ScalarFunction, ScalarFunctionBuilder};
    use autocxx::WithinUniquePtr;
    use cxx::{let_cxx_string, UniquePtr};
    use std::pin::Pin;

    fn bind(
        context: &ClientContext,
        func: &ScalarFunction,
        args: &mut [UniquePtr<Expression>],
    ) -> UniquePtr<RustFunctionData> {
        UniquePtr::null()
    }
    fn function(
        args: &DataChunk,
        state: &ExpressionState,
        output: Pin<&mut Vector>,
    ) -> UniquePtr<PreservedError> {
        UniquePtr::null()
    }

    #[test]
    fn test_basic() {
        let_cxx_string!(function_name = "test_function");

        let return_type = LogicalTypeId::VARCHAR;

        unsafe {
            let mut sfb =
                ScalarFunctionBuilder::new(function_name, return_type).within_unique_ptr();

            sfb.pin_mut().set_bind(bind);
            sfb.pin_mut().set_function(function);

            sfb.pin_mut().build();
        }
    }
}
