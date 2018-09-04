extern crate libc;
extern crate llvm_sys;

mod hello_world;
use hello_world::*;

mod llvm_builder;
use llvm_builder::*;

mod utils;

fn main() {
    let mut llvm_builder = LlvmBuilder::new("");
    llvm_builder.setup_main();
    
    // create hello world llvm ir
    hello_world(llvm_builder.builder, llvm_builder.context, llvm_builder.module);

    llvm_builder.return_main();

    llvm_builder.dump();
    llvm_builder.emit_file("hello_world.ll");
}
