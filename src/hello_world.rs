use std::ffi::CString;
use llvm_sys::core::*;
use llvm_sys::*;

use utils::*;

pub fn hello_world(
    builder: *mut LLVMBuilder,
    context: *mut LLVMContext,
    module: *mut LLVMModule,
) {
    unsafe {
        let print = create_printf(module);
        let mut printf_args = [codegen_string(builder, context, "hello world\n\r")];

        LLVMBuildCall(
            builder,
            print,
            printf_args.as_mut_ptr(),
            1,
            CString::new("").unwrap().as_ptr(),
        );
    }
}
