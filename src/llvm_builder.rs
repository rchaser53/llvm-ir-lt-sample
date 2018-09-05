use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::*;

pub struct LlvmBuilder {
    pub builder: *mut LLVMBuilder,
    pub context: *mut LLVMContext,
    pub module: *mut LLVMModule,
}

impl LlvmBuilder {
    pub fn new(module_name: &str) -> LlvmBuilder {
        unsafe {
            let context = LLVMGetGlobalContext();
            let mod_name = CString::new(module_name).unwrap();

            LlvmBuilder {
                builder: LLVMCreateBuilder(),
                module: LLVMModuleCreateWithName(mod_name.as_ptr()),
                context: context,
            }
        }
    }

    pub fn setup_main(&mut self) -> *mut LLVMValue {
        let main = unsafe { self.add_function(LLVMInt32Type(), &mut [], "main") };
        let block = self.append_basic_block("main", "entry");
        self.end_basic_block(block);
        main
    }

    pub fn add_function(
        &mut self,
        ret_type: *mut LLVMType,
        args: &mut [*mut LLVMType],
        fn_name: &str,
    ) -> *mut LLVMValue {
        unsafe {
            let fn_type = LLVMFunctionType(ret_type, args.as_mut_ptr(), args.len() as u32, 0);
            let cstring = CString::new(fn_name).unwrap();
            let ptr = cstring.as_ptr() as *mut _;
            LLVMAddFunction(self.module, ptr, fn_type)
        }
    }

    pub fn get_named_function(&mut self, name: &str) -> *mut LLVMValue {
        let func_name = CString::new(name).unwrap();
        unsafe { LLVMGetNamedFunction(self.module, func_name.as_ptr()) }
    }

    pub fn append_basic_block(&mut self, function_name: &str, name: &str) -> *mut LLVMBasicBlock {
        let label_name = CString::new(name).unwrap();
        let function = self.get_named_function(function_name);

        unsafe { LLVMAppendBasicBlock(function, label_name.as_ptr()) }
    }

    pub fn end_basic_block(&mut self, block: *mut LLVMBasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, block);
        }
    }

    pub fn return_main(&mut self) {
        unsafe {
            LLVMBuildRet(self.builder, LLVMConstInt(LLVMInt32Type(), 0, 0));
        }
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.module) }
    }

    pub fn emit_file(&self, filename: &str) {
        unsafe {
            LLVMPrintModuleToFile(
                self.module,
                CString::new(filename).unwrap().as_ptr(),
                CString::new("").unwrap().as_ptr() as *mut _,
            );
        }
    }
}

impl Drop for LlvmBuilder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
        }
    }
}
