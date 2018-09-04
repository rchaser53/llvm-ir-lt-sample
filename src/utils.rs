use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

pub fn codegen_string(
    builder: *mut LLVMBuilder,
    context: *mut LLVMContext,
    input_str: &str,
) -> *mut LLVMValue {
    let length = input_str.len() as u32;
    unsafe {
        let str_val =
            LLVMConstStringInContext(context, CString::new(input_str).unwrap().as_ptr(), length - 1, 0);
        let llvm_value = LLVMBuildAlloca(
            builder,
            LLVMArrayType(LLVMInt8Type(), length),
            CString::new("").unwrap().as_ptr(),
        );
        LLVMBuildStore(builder, str_val, llvm_value);

        let mut args = [
            LLVMConstInt(LLVMInt32Type(), 0, 0),
            LLVMConstInt(LLVMInt32Type(), 0, 0),
        ];

        return LLVMBuildGEP(
            builder,
            llvm_value,
            args.as_mut_ptr(),
            2,
            CString::new("").unwrap().as_ptr(),
        );
    }
}

pub fn create_printf(module: *mut LLVMModule) -> *mut LLVMValue {
    unsafe {
        let mut printf_args_type_list = [LLVMPointerType(LLVMInt8Type(), 0)];
        let printf_type =
            LLVMFunctionType(LLVMInt32Type(), printf_args_type_list.as_mut_ptr(), 1, 0);
        return LLVMAddFunction(
            module,
            CString::new("printf").unwrap().as_ptr() as *mut _,
            printf_type,
        );
    }
}
