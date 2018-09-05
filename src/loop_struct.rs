use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

use llvm_builder::*;
use utils::*;

#[macro_export]
macro_rules! c_string {
    ($w:expr) => {
        CString::new($w).unwrap().as_ptr()
    };
}

pub fn loop_struct(lb: &mut LlvmBuilder) {
    unsafe {
        let mut named = LLVMStructCreateNamed(lb.context, c_string!("test_struct"));
        let mut elements = [LLVMInt32Type(), LLVMInt32Type()];
        LLVMStructSetBody(named, elements.as_mut_ptr(), 2, 0);

        let mut struct_test_func_types = [LLVMPointerType(named, 0), LLVMInt32Type()];
        let func_type =
            LLVMFunctionType(LLVMInt32Type(), struct_test_func_types.as_mut_ptr(), 2, 0);
        let struct_test_func = LLVMAddFunction(lb.module, c_string!("struct_test_func"), func_type);
        let func_entry = LLVMAppendBasicBlock(struct_test_func, c_string!("entry"));
        LLVMPositionBuilderAtEnd(lb.builder, func_entry);

        let target_struct = LLVMGetParam(struct_test_func, 0);

        set_field_value(
            lb.builder,
            target_struct,
            0,
            LLVMConstInt(LLVMInt32Type(), 12, 0),
        );
        let mut field = get_field_value(lb.builder, target_struct, 0);

        let struct_test_func_ret = LLVMBuildAdd(
            lb.builder,
            field,
            LLVMGetParam(struct_test_func, 1),
            c_string!(""),
        );
        LLVMBuildRet(lb.builder, struct_test_func_ret);

        /**/
        let mut param_types = [LLVMInt32Type()];
        let mut ret_type = LLVMFunctionType(LLVMInt32Type(), param_types.as_mut_ptr(), 1, 0);

        let mut main = LLVMAddFunction(lb.module, c_string!("main"), ret_type);
        let mut entry = LLVMAppendBasicBlock(main, c_string!("entry"));

        let mut loop_block = LLVMAppendBasicBlockInContext(lb.context, main, c_string!("loop"));
        let mut loop_end_block =
            LLVMAppendBasicBlockInContext(lb.context, main, c_string!("loop_end"));

        LLVMPositionBuilderAtEnd(lb.builder, entry);
        /**/

        let mut llvm_printf_int = create_printf_int(lb.module);

        let mut val = LLVMBuildAlloca(lb.builder, LLVMInt32Type(), c_string!(""));
        LLVMBuildStore(lb.builder, LLVMConstInt(LLVMInt32Type(), 0, 0), val);

        LLVMBuildBr(lb.builder, loop_block);
        LLVMPositionBuilderAtEnd(lb.builder, loop_block);

        // // loop
        let mut load_val = LLVMBuildLoad(lb.builder, val, c_string!(""));
        let mut lhs = LLVMBuildAdd(
            lb.builder,
            LLVMConstInt(LLVMInt32Type(), 1, 0),
            load_val,
            c_string!("a.b"),
        );
        LLVMBuildStore(lb.builder, lhs, val);

        let mut printf_args = [
            codegen_string(lb.builder, lb.context, "%d\n\r"),
            LLVMBuildLoad(lb.builder, val, c_string!("")),
        ];

        let mut for_cond = LLVMBuildICmp(
            lb.builder,
            LLVMIntPredicate::LLVMIntUGT,
            lhs,
            LLVMConstInt(LLVMInt32Type(), 2, 0),
            c_string!(""),
        );

        LLVMBuildCall(
            lb.builder,
            llvm_printf_int,
            printf_args.as_mut_ptr(),
            2,
            c_string!(""),
        );

        LLVMBuildCondBr(lb.builder, for_cond, loop_end_block, loop_block);
        LLVMPositionBuilderAtEnd(lb.builder, loop_end_block);

        // // loop end
        LLVMBuildRet(lb.builder, LLVMConstInt(LLVMInt32Type(), 0, 0));

        lb.emit_file("loop_struct.ll");
    }
}
