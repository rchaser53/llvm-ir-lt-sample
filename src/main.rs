extern crate libc;
extern crate llvm_sys;

mod hello_world;
use hello_world::*;

mod llvm_builder;
use llvm_builder::*;

mod loop_struct;
use loop_struct::*;

mod utils;

fn main() {
    let mut llvm_builder = LlvmBuilder::new("");

    /*
      you can enjoy below llvm ir emit function
      please uncomment what you want.
    */ 

    // create hello world llvm ir
    hello_world(&mut llvm_builder);

    // create loop struct llvm ir
    // loop_struct(&mut llvm_builder);

    llvm_builder.dump();
}
