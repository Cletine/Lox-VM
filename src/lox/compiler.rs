//
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;

#[derive(Debug)]
pub enum CompilerError {
    LLVMError(String),
    UndefinedVariable(String),
    TypeMismatch(String),
}

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    // also eventually put symbol table/scopes here
}

impl<'ctx> StmtVisitor<Result<(), CompilerError>> for CodeGen<'ctx> {
    // Implement statement compilation methods here (e.g., generating functions, blocks)
    // Most statement implementations will return Ok(()) as they emit instructions into the block
}

impl<'ctx> ExprVisitor<Result<inkwell::values::BasicValueEnum<'ctx>, CompilerError>> for CodeGen<'ctx> {
    // Implement expression compilation methods here
    // Most expressions will return Ok(BasicValueEnum) representing the computed LLVM value
}

pub fn compile_program(program: &[Box<Statement>]) -> Result<(), CompilerError> {
    let context = Context::create();
    let module = context.create_module("my_program");
    let builder = context.create_builder();

    // Instantiate your code generator pass
    let mut codegen = CodeGen {
        context: &context,
        module,
        builder,
    };

    // Process the list of statements sequentially
    for stmt in program {
        // Dispatch the statement to the visitor framework
        stmt.accept(&mut codegen)?;
    }

    // Verify the generated LLVM module for structural validity
    codegen.module.verify().map_err(|e| CompilerError::LLVMError(e.to_string()))?;

    // Print the generated LLVM IR to stdout for debugging
    codegen.module.print_to_stderr();

    Ok(())
}

