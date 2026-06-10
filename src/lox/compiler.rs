use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;
use inkwell::values::FloatValue;
use inkwell::values::IntValue;
use inkwell::values::PointerValue;
use inkwell::AddressSpace;
use crate::lox::Token;
use crate::lox::Object;
use crate::lox::Expr;
use crate::lox::Statement;
use crate::lox::StmtVisitor;
use crate::lox::ExprVisitor;
use crate::CompilerError;
use crate::lox::Environment;
use crate::lox::VariableInfo;


pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub environment: Environment<'ctx>,
    // also eventually put symbol table/scopes here
}

impl<'ctx> StmtVisitor<'ctx, Result<(), CompilerError>> for CodeGen<'ctx> {
    // Implement statement compilation methods here (e.g., generating functions, blocks)
    // Most statement implementations will return Ok(()) as they emit instructions into the blocks
    fn visit_block_stmt(&mut self, statements: &'ctx Vec<Box<Statement>>) -> Result<(), CompilerError> {
        todo!()
    }

    fn visit_expr_stmt(&mut self, expression: &'ctx Expr) -> Result<(), CompilerError>  {
        todo!()
    }
    fn visit_function_stmt(&mut self, name: &'ctx Token, arguments: &'ctx Vec<Box<Expr>>, body: &'ctx Vec<Box<Statement>>) -> Result<(), CompilerError> {
        todo!()
    }
    fn visit_if_stmt(&mut self, condition: &'ctx Expr, then_branch: &'ctx Statement, else_branch: &'ctx Option<Statement>) -> Result<(), CompilerError> {
        todo!()
    }

    fn visit_return_stmt(&mut self, keyword: &'ctx Token, value: &'ctx Expr) -> Result<(), CompilerError> {
        todo!()
    }

    fn visit_while_stmt(&mut self, condition: &'ctx Expr, body: &'ctx Statement) -> Result<(), CompilerError> {
        todo!()
    }

    fn visit_var_stmt(&mut self, name: &'ctx Token, initializer: &'ctx Expr) -> Result<(), CompilerError> {
        todo!()
    }
}

impl<'ctx> ExprVisitor<'ctx, Result<inkwell::values::BasicValueEnum<'ctx>, CompilerError>> for CodeGen<'ctx> {
    // Implement expression compilation methods here
    // Most expressions will return Ok(BasicValueEnum) representing the computed LLVM value
    fn visit_binary_expr(&mut self, left: &'ctx Expr, operator: &'ctx Token, right: &'ctx Expr) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        todo!()
    }


    fn visit_call_expr(&mut self, callee: &'ctx Expr, paren: &'ctx Token, arguments: &'ctx Vec<Box<Expr>>) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        todo!()
    }


    fn visit_grouping_expr(&mut self, expression: &'ctx Expr) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        todo!()
    }



    fn visit_logical_expr(&mut self, left: &'ctx Expr, operator: &'ctx Token, right: &'ctx Expr) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        todo!()
    }




    fn visit_assign_expr(&mut self, name: &'ctx Token, value: &'ctx Expr) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        todo!()
    }

    fn visit_variable_expr(&mut self, name: &'ctx Token) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        match self.environment.lookup(name) {
            Ok(VariableInfo {pointer: ptr, ty: t, is_mutable: mutbl}) => {
               todo!() 
            }
            Err(compiler_error) =>  return Err(compiler_error),
        }
    }


    fn visit_unary_expr(&mut self, operator: &'ctx Token, right: &'ctx Expr) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_literal_expr(&mut self, value: &'ctx Object) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        match value {
            Object::STRING(string) => self.literal_string_expr_node(string),
            Object::NUMBER(number) => self.literal_number_expr_node(*number),
            Object::NULL => self.literal_null_expr_node(),
            Object::BOOL(boolean) => self.literal_bool_expr_node(*boolean),
        }
    }
}

impl <'ctx> CodeGen<'ctx> {
    fn literal_number_expr_node(&mut self, value: f64) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let f64_type = self.context.f64_type();
        let constant_fp: FloatValue<'ctx> = f64_type.const_float(value);
        return Ok(BasicValueEnum::FloatValue(constant_fp))
    }

    fn literal_null_expr_node(&mut self) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let null_ptr: PointerValue<'ctx> = ptr_type.const_null();
        return Ok(BasicValueEnum::PointerValue(null_ptr))
    }

    fn literal_bool_expr_node(&mut self, value: bool) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let bool_type = self.context.bool_type();
        let constant_bool: IntValue <'ctx> = bool_type.const_int(value as u64, false);
        return Ok(BasicValueEnum::IntValue(constant_bool))
    }

    fn literal_string_expr_node(&mut self, value: &str) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let global_string_ptr = self.builder.build_global_string_ptr(value,"str_literal")
            .map_err(|_| CompilerError::LLVMError("Failed to build global string".to_string()))?;
        let basic_value: BasicValueEnum<'ctx> = global_string_ptr.as_pointer_value().into();
        return Ok(basic_value)
    }
}


pub fn compile_program(program: &Vec<Statement>, file: &str) -> Result<(), CompilerError> {
    let context = Context::create();
    let module = context.create_module(file);
    let builder = context.create_builder();
    let environment = Environment::new();

    // Instantiate your code generator pass
    let mut codegen = CodeGen {
        context: &context,
        module,
        builder,
        environment,
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

