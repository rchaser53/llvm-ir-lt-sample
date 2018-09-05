extern crate libc;
extern crate llvm_sys;

mod hello_world;
use hello_world::*;

mod llvm_builder;
use llvm_builder::*;

mod utils;

fn main() {
    let mut llvm_builder = LlvmBuilder::new("");
    
    // create hello world llvm ir
    hello_world(&mut llvm_builder);

    llvm_builder.dump();
    llvm_builder.emit_file("hello_world.ll");
}
