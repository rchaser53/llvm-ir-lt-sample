use llvm_sys::core::*;
use std::ffi::CString;

use llvm_builder::*;
use utils::*;

pub fn hello_world(lb: &mut LlvmBuilder) {
    unsafe {
        lb.setup_main();
        let print = create_printf(lb.module);
        let mut printf_args = [codegen_string(lb.builder, lb.context, "hello world\n\r")];

        LLVMBuildCall(
            lb.builder,
            print,
            printf_args.as_mut_ptr(),
            printf_args.len() as u32,
            CString::new("").unwrap().as_ptr(),
        );

        lb.return_main();

        lb.emit_file("hello_world.ll");
    }
}
