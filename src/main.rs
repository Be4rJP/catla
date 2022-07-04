mod vm;
mod syntax;

extern crate llvm_sys as llvm;
#[macro_use] extern crate anyhow;

use std::ffi::CString;
use std::mem;
use std::mem::MaybeUninit;
use std::ops::Add;
use std::sync::Arc;

use llvm::core::*;
use llvm::analysis::{LLVMVerifyModule, LLVMVerifierFailureAction};
use llvm::target::{LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget};
use llvm::target_machine::*;
use llvm::*;

use anyhow::{Context, Result};
use llvm::execution_engine::{LLVMCreateExecutionEngineForModule, LLVMDisposeExecutionEngine, LLVMGetFunctionAddress, LLVMLinkInMCJIT};
use syntax::lexer;
use crate::lexer::Lexer;

fn initialize() {
    syntax::lexer::NONE_SPACE.clone();
    syntax::lexer::NONE_STRING.clone();
    syntax::lexer::EXPRESSION_SPLIT.clone();
    syntax::lexer::OPERATOR_PLUS.clone();
    syntax::lexer::OPERATOR_MINUS.clone();
    syntax::lexer::OPERATOR_MULTIPLY.clone();
    syntax::lexer::OPERATOR_DIVISION.clone();
    syntax::lexer::OPERATOR_REMAINDER.clone();
    syntax::lexer::OPERATOR_ASSIGNMENT.clone();
    syntax::lexer::DOT.clone();
}

fn main() {
    initialize();
    /*
    unsafe {
        // Set up a context, module and builder in that context.
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithNameInContext(b"sum\0".as_ptr() as *const _, context);
        let builder = LLVMCreateBuilderInContext(context);

        // get a type for sum function
        let i64t = LLVMInt64TypeInContext(context);
        let mut argts = [i64t, i64t, i64t];
        let function_type = LLVMFunctionType(i64t, argts.as_mut_ptr(), argts.len() as u32, 0);

        // add it to our module
        let function = LLVMAddFunction(module, b"sum\0".as_ptr() as *const _, function_type);

        // Create a basic block in the function and set our builder to generate
        // code in it.
        let bb = LLVMAppendBasicBlockInContext(context, function, b"entry\0".as_ptr() as *const _);

        LLVMPositionBuilderAtEnd(builder, bb);

        // get the function's arguments
        let x = LLVMGetParam(function, 0);
        let y = LLVMGetParam(function, 1);
        let z = LLVMGetParam(function, 2);

        let sum = LLVMBuildAdd(builder, x, y, b"sum.1\0".as_ptr() as *const _);
        let sum = LLVMBuildAdd(builder, sum, z, b"sum.2\0".as_ptr() as *const _);
        let sum = LLVMBuildAdd(builder, sum, x, b"sum.3\0".as_ptr() as *const _);

        // Emit a `ret void` into the function
        LLVMBuildRet(builder, sum);

        // done building
        LLVMDisposeBuilder(builder);

        // Dump the module as IR to stdout.
        LLVMDumpModule(module);

        // build an execution engine
        let mut ee = mem::uninitialized();
        let mut out = mem::zeroed();

        // robust code should check that these calls complete successfully
        // each of these calls is necessary to setup an execution engine which compiles to native
        // code
        LLVMLinkInMCJIT();
        LLVM_InitializeNativeTarget();
        LLVM_InitializeNativeAsmPrinter();

        // takes ownership of the module
        LLVMCreateExecutionEngineForModule(&mut ee, module, &mut out);

        let addr = LLVMGetFunctionAddress(ee, b"sum\0".as_ptr() as *const _);

        let f: extern "C" fn(u64, u64, u64) -> u64 = mem::transmute(addr);

        let x: u64 = 20;
        let y: u64 = 500;
        let z: u64 = 1000;
        let res = f(x, y, z);

        println!("{} + {} + {} = {}", x, y, z, res);

        // Clean up the rest.
        LLVMDisposeExecutionEngine(ee);
        LLVMContextDispose(context);
    }*/

    let text = "var test = 200.0;var ああ=2000;200.00.0;";

    let lexer = Lexer::new(text.to_string());
    let tokens = lexer.scan();

    let mut buffer = "".to_string();
    for token in tokens {
        buffer += token.word.as_str();
        buffer += ", ";
    }

    println!("original : {}", &text);
    println!("token : {}", buffer.as_str());

}