//
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace::default;

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
    pub environment: Environment<'ctx>,
    // also eventually put symbol table/scopes here
}

impl<'ctx> StmtVisitor<Result<(), CompilerError>> for CodeGen<'ctx> {
    // Implement statement compilation methods here (e.g., generating functions, blocks)
    // Most statement implementations will return Ok(()) as they emit instructions into the blocks


}

impl<'ctx> ExprVisitor<Result<inkwell::values::BasicValueEnum<'ctx>, CompilerError>> for CodeGen<'ctx> {
    // Implement expression compilation methods here
    // Most expressions will return Ok(BasicValueEnum) representing the computed LLVM value
    //

    fn visit_variable_expr(&mut self, name: &Token) {
        match self.environment.lookup(&Token) {
            Ok() => todo!(),
            None => todo!(), 
                //UndefinedVariable(format!("Undefined Variable: {}", name.lexeme)),
        }
    }

    fn visit_literal_expr(&mut self, value: &Object) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        match value {
            Object::STRING(string) => literal_string_expr_node(string),
            Object::NUMBER(number) => self.literal_number_expr_node(number),
            Object::NULL => self.literal_null_expr_node(),
            Object::BOOL(boolean) => self.literal_bool_expr_node(boolean),
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
        let ptr_type = self.context.ptr_type(default());
        let null_ptr: PointerValue<'ctx> = ptr_type.const_null();
        return Ok(BasicValueEnum::FloatValue(null_ptr))
    }

    fn literal_bool_expr_node(&mut self, value: bool) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let bool_type = self.context.bool_type();
        let constant_bool: IntValue <'ctx> = bool_type.const_int(value as u64, false);
        return Ok(BasicValueEnum::IntValue(constant_bool))
    }

    fn literal_string_expr_node(&mut self, value: String) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let global_string_ptr = self.builder.build_global_string_ptr(value.as_str(),"str_literal")
            .map_err(|_| CompilerError::LLVMError("Failed to build global string".to_string()))?;
        let basic_value: BasicValueEnum<'ctx> = global_string_ptr.as_pointer_value().into();
        Ok(basic_value)
    }
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

