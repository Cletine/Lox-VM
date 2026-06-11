use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;
use inkwell::values::FloatValue;
use inkwell::values::IntValue;
use inkwell::values::PointerValue;
use inkwell::IntPredicate;
use inkwell::AddressSpace;
use crate::core::Token;
use crate::core::TokenType;
use crate::core::Object;
use crate::core::Expr;
use crate::core::Statement;
use crate::core::StmtVisitor;
use crate::core::ExprVisitor;
use crate::CompilerError;
use crate::core::Environment;
use crate::core::VariableInfo;


pub struct CodeGen<'env, 'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub environment: &'env Environment<'ctx>,
    // also eventually put symbol table/scopes here
}

enum OpValues<'ctx> {
    Float(FloatValue<'ctx>, FloatValue<'ctx>),
    Int(IntValue<'ctx>, IntValue<'ctx>),
    Pointer(PointerValue<'ctx>, PointerValue<'ctx>),
    TypeMismatch,
}

impl<'env, 'ctx> StmtVisitor<'ctx, Result<(), CompilerError>> for CodeGen<'env, 'ctx> {
    // Implement statement compilation methods here (e.g., generating functions, blocks)
    // Most statement implementations will return Ok(()) as they emit instructions into the blocks
    fn visit_block_stmt(&mut self, statements: &Vec<Box<Statement>>) -> Result<(), CompilerError> {
        todo!()
    }

    fn visit_expr_stmt(&mut self, expression: &Expr) -> Result<(), CompilerError>  {
        todo!()
    }
    fn visit_function_stmt(&mut self, name: &Token, arguments: &Vec<Box<Expr>>, body: &Vec<Box<Statement>>) -> Result<(), CompilerError> {
        todo!()
    }
    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Statement, else_branch: &Option<Statement>) -> Result<(), CompilerError> {
        todo!()
    }

    fn visit_return_stmt(&mut self, keyword: &Token, value: &Expr) -> Result<(), CompilerError> {
        todo!()
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Statement) -> Result<(), CompilerError> {
        todo!()
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<(), CompilerError> {
        todo!()
    }
}

impl<'env, 'ctx> ExprVisitor<'ctx, Result<inkwell::values::BasicValueEnum<'ctx>, CompilerError>> for CodeGen<'env, 'ctx> {
    // Implement expression compilation methods here
    // Most expressions will return Ok(BasicValueEnum) representing the computed LLVM value
    


    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Box<Expr>>) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        todo!()
    }




    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        todo!()
    }



    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        todo!()
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        match self.environment.lookup(name) {
            Ok(VariableInfo {pointer: ptr, ty: basic_enum_type, is_mutable: mutbl}) => {
                let variable_val = self.builder.build_load(*basic_enum_type, *ptr, &format!("load_{}", name.lexeme))
                    .map_err(|_| CompilerError::LLVMError("Failed to load variable".to_string()))?;
                    return Ok(variable_val)
            }
            Err(compiler_error) =>  return Err(compiler_error),
        }
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        return expression.accept(self)
    }

    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        let left_expr = left.accept(self)?; 
        let right_expr = right.accept(self)?; 

        let operands = match (left_expr, right_expr) {
            (BasicValueEnum::FloatValue(lhs), BasicValueEnum::FloatValue(rhs)) => OpValues::Float(lhs, rhs),
            (BasicValueEnum::IntValue(lhs), BasicValueEnum::IntValue(rhs)) => OpValues::Int(lhs, rhs),
            (BasicValueEnum::PointerValue(lhs), BasicValueEnum::PointerValue(rhs)) => OpValues::Pointer(lhs, rhs),
            _ => return Err(CompilerError::LLVMError("Type mismatch between binary operands".to_string())),
        };

        match operator.token_type { 
            TokenType::Minus => {
                match operands {
                    OpValues::Float(lhs,rhs) => {
                        let float_sub_sum = self.builder.build_float_sub(lhs, rhs, "sub_float")
                            .map_err(|_| CompilerError::LLVMError("Failed to subtract float values".to_string()))?;
                        Ok(BasicValueEnum::FloatValue(float_sub_sum))
                    }
                    OpValues::Int(lhs, rhs) => {
                        let int_sub_sum = self.builder.build_int_add(lhs, rhs, "add_int")
                            .map_err(|_| CompilerError::LLVMError("Integer subtraction failed".to_string()))?;
                        Ok(BasicValueEnum::IntValue(int_sub_sum))
                    },
                    _ => return Err(CompilerError::LLVMError("Invalid operand types  for '+' ".to_string())),
                }
            }, 
            TokenType::Slash => { 
                match operands {
                    OpValues::Float(lhs,rhs) => {
                        let float_div_sum = self.builder.build_float_div(lhs, rhs, "div_float") 
                            .map_err(|_| CompilerError::LLVMError("Failed to divide float values".to_string()))?;
                        Ok(BasicValueEnum::FloatValue(float_div_sum))
                    },
                    OpValues::Int(lhs, rhs) => {
                        let int_div_sum = self.builder.build_int_signed_div(lhs, rhs, "add_int")
                            .map_err(|_| CompilerError::LLVMError("Integer division failed".to_string()))?;
                        Ok(BasicValueEnum::IntValue(int_div_sum))
                    },
                    _ => return Err(CompilerError::LLVMError("Invalid operand types  for '/' ".to_string())),
                }
            }, 
            TokenType::Star => { 
                match operands {
                    OpValues::Float(lhs,rhs) => {
                        let mul_sum = self.builder.build_float_mul(lhs, rhs, "mul_float") 
                            .map_err(|_| CompilerError::LLVMError("Failed to multiply float values".to_string()))?;
                        Ok(BasicValueEnum::FloatValue(mul_sum))
                    },
                    OpValues::Int(lhs, rhs) => {
                        let int_mul_sum = self.builder.build_int_mul(lhs, rhs, "add_int")
                            .map_err(|_| CompilerError::LLVMError("Integer multiplication failed".to_string()))?;
                        Ok(BasicValueEnum::IntValue(int_mul_sum))
                    },
                    _ => return Err(CompilerError::LLVMError("Invalid operand types  for '*' ".to_string())),
                }
            }, 

            TokenType::Plus => { 
                match operands {
                    OpValues::Float(lhs, rhs) => {
                        let add_sum = self.builder.build_float_add(lhs, rhs, "add_float")
                            .map_err(|_| CompilerError::LLVMError("Failed to add float values".to_string()))?;
                        Ok(BasicValueEnum::FloatValue(add_sum))
                    },
                    OpValues::Int(lhs, rhs) => {
                        let int_add_sum = self.builder.build_int_add(lhs, rhs, "add_int")
                            .map_err(|_| CompilerError::LLVMError("Integer addtion failed".to_string()))?;
                        Ok(BasicValueEnum::IntValue(int_add_sum))
                    },
                    _ => return Err(CompilerError::LLVMError("Invalid operand types  for '+' ".to_string())),
                }
            },

            TokenType::EqualEqual => match operands {
                OpValues::Float(lhs, rhs) => {
                    let float_equals = self.builder.build_float_compare(inkwell::FloatPredicate::OEQ, lhs, rhs, "feq")
                        .map_err(|_| CompilerError::LLVMError("Float comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(float_equals))
                },
                OpValues::Int(lhs, rhs) => {
                    let int_equals = self.builder.build_int_compare(inkwell::IntPredicate::EQ, lhs, rhs, "ieq")
                        .map_err(|_| CompilerError::LLVMError("Integer comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(int_equals))
                },
                OpValues::Pointer(lhs, rhs) => {
                    let pointer_equals = self.builder.build_int_compare(inkwell::IntPredicate::EQ, lhs, rhs, "ptreq")
                        .map_err(|_| CompilerError::LLVMError("Pointer comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(pointer_equals))
                },

                _ => Err(CompilerError::LLVMError("Operator '==' not supported for these types".to_string())),
            },

            TokenType::BangEqual => match operands {
                OpValues::Float(lhs, rhs) => {
                    let val = self.builder.build_float_compare(inkwell::FloatPredicate::ONE, lhs, rhs, "feq")
                        .map_err(|_| CompilerError::LLVMError("Float comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(val))
                },
                OpValues::Int(lhs, rhs) => {
                    let val = self.builder.build_int_compare(inkwell::IntPredicate::NE, lhs, rhs, "ieq")
                        .map_err(|_| CompilerError::LLVMError("Integer comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(val))
                },
                OpValues::Pointer(lhs, rhs) => {
                    let val = self.builder.build_int_compare(inkwell::IntPredicate::NE, lhs, rhs, "ptreq")
                        .map_err(|_| CompilerError::LLVMError("Pointer comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(val))
                },

                _ => Err(CompilerError::LLVMError("Operator '!=' not supported for these types".to_string())),
            },

            TokenType::Less => match operands {
                OpValues::Float(lhs, rhs) => {
                    let float_less_than = self.builder.build_float_compare(inkwell::FloatPredicate::OLT, lhs, rhs, "flt")
                        .map_err(|_| CompilerError::LLVMError("Float comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(float_less_than))
                },
                OpValues::Int(lhs, rhs) => {
                    // Assumes signed integers; change to ULT if your booleans/ints are unsigned
                    let int_less_than = self.builder.build_int_compare(inkwell::IntPredicate::SLT, lhs, rhs, "ilt")
                        .map_err(|_| CompilerError::LLVMError("Integer comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(int_less_than))
                },
                _ => Err(CompilerError::LLVMError("Operator '<' not supported for these types".to_string())),
            },

            TokenType::LessEqual => match operands {
                OpValues::Float(lhs, rhs) => {
                    let float_less_equal = self.builder.build_float_compare(inkwell::FloatPredicate::OLE, lhs, rhs, "flt")
                        .map_err(|_| CompilerError::LLVMError("Float comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(float_less_equal))
                },
                OpValues::Int(lhs, rhs) => {
                    // Assumes signed integers; change to ULT if your booleans/ints are unsigned
                    let int_less_equal = self.builder.build_int_compare(inkwell::IntPredicate::SLE, lhs, rhs, "ilt")
                        .map_err(|_| CompilerError::LLVMError("Integer comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(int_less_equal))
                },
                _ => Err(CompilerError::LLVMError("Operator '<=' not supported for these types".to_string())),
            },

            TokenType::Greater => match operands {
                OpValues::Float(lhs, rhs) => {
                    let float_greater_than = self.builder.build_float_compare(inkwell::FloatPredicate::OGT, lhs, rhs, "flt")
                        .map_err(|_| CompilerError::LLVMError("Float comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(float_greater_than))
                },
                OpValues::Int(lhs, rhs) => {
                    // Assumes signed integers; change to ULT if your booleans/ints are unsigned
                    let int_greater_than = self.builder.build_int_compare(inkwell::IntPredicate::SGT, lhs, rhs, "ilt")
                        .map_err(|_| CompilerError::LLVMError("Integer comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(int_greater_than))
                },
                _ => Err(CompilerError::LLVMError("Operator '<' not supported for these types".to_string())),
            },

            TokenType::GreaterEqual => match operands {
                OpValues::Float(lhs, rhs) => {
                    let float_greater_equal = self.builder.build_float_compare(inkwell::FloatPredicate::OGE, lhs, rhs, "flt")
                        .map_err(|_| CompilerError::LLVMError("Float comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(float_greater_equal))
                },
                OpValues::Int(lhs, rhs) => {
                    // Assumes signed integers; change to ULT if your booleans/ints are unsigned
                    let int_greater_equal = self.builder.build_int_compare(inkwell::IntPredicate::SGE, lhs, rhs, "ilt")
                        .map_err(|_| CompilerError::LLVMError("Integer comparison failed".to_string()))?;
                    Ok(BasicValueEnum::IntValue(int_greater_equal))
                },
                _ => Err(CompilerError::LLVMError("Operator '<' not supported for these types".to_string())),
            },
            _ => Err(CompilerError::LLVMError("Unsupported Operator on Binary Expression".to_string())), 
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let right_expr = right.accept(self)?;
        match operator.token_type {
            TokenType::Minus => { 
                match right_expr {
                    BasicValueEnum::IntValue(int_val) => {
                        let int_negated = self.builder.build_int_neg(int_val, "neg_int")
                            .map_err(|_| CompilerError::LLVMError("Failed to convert int value to negative int value".to_string()))?;
                        return Ok(BasicValueEnum::IntValue(int_negated))
                    },
                    BasicValueEnum::FloatValue(float_val) => {
                        let float_negated = self.builder.build_float_neg(float_val, "neg_float")
                            .map_err(|_| CompilerError::LLVMError("Failed to convert float value to negative float value".to_string()))?;
                        return Ok(BasicValueEnum::FloatValue(float_negated))
                    },
                    _ => return Err(CompilerError::LLVMError("Invalid Unary Operator '-' on given type".to_string())),
                }
            },
            TokenType::Bang => { 
                match right_expr {
                    BasicValueEnum::IntValue(int_val) => {
                        let zero = int_val.get_type().const_int(0, false);
                        let is_zero = self.builder.build_int_compare(IntPredicate::EQ, int_val, zero, "is_zero")
                            .map_err(|_| CompilerError::LLVMError("Failed to convert float value to negative float value".to_string()))?;
                        return Ok(BasicValueEnum::IntValue(is_zero))
                    },
                    _ => return Err(CompilerError::LLVMError("Invalid Unary Operator '!' on given type".to_string()))
                }
            },
            _ => return Err(CompilerError::LLVMError("Unsupported Operator on Unary expression".to_string())),
        }
    }

    fn visit_literal_expr(&mut self, value: &Object) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        match value {
            Object::STRING(string) => self.literal_string_expr_node(string),
            Object::NUMBER(number) => self.literal_number_expr_node(*number),
            Object::NULL => self.literal_null_expr_node(),
            Object::BOOL(boolean) => self.literal_bool_expr_node(*boolean),
        }
    }
}

impl <'env, 'ctx> CodeGen<'env, 'ctx> {
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
        environment: &environment,
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

