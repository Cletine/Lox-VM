use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;
use inkwell::values::FloatValue;
use inkwell::values::IntValue;
use inkwell::values::PointerValue;
use inkwell::values::FunctionValue;
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
    pub environment: &'env mut Environment<'ctx>,
    pub fn_value_opt: Option<FunctionValue<'ctx>>,
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
        self.environment.enter_scope(); 
        for stmt in statements {
            // Dispatch the statement to the visitor framework
            stmt.accept(self)?;
        }
        self.environment.exit_scope();
        Ok(())
    }

    fn visit_expr_stmt(&mut self, expression: &Expr) -> Result<(), CompilerError>  {
        expression.accept(self)?;
        Ok(())
    }

    fn visit_function_stmt(&mut self, name: &Token, arguments: &Vec<Box<Expr>>, body: &Vec<Box<Statement>>) -> Result<(), CompilerError> {
        todo!()
    }

    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Statement, else_branch: &Option<Statement>) -> Result<(), CompilerError> {
        let parent = self.fn_value_opt.unwrap();
        let zero_const = self.context.f64_type().const_float(0.0);

        let cond_expr = condition.accept(self)?;
        let cond = self.is_truthy(cond_expr)?;

        // build branch
        let then_bb = self.context.append_basic_block(parent, "then");
        let cont_bb = self.context.append_basic_block(parent, "ifcont");

        let else_bb = if else_branch.is_some() {
            Some(self.context.append_basic_block(parent, "else"))
        }
        else {
            None
        };

        // Branch to 'else_bb' if it exists, otherwise branch directly to 'cont_bb'
        let false_target = else_bb.unwrap_or(cont_bb);
        self.builder.build_conditional_branch(cond, then_bb, false_target)
            .map_err(|_| CompilerError::LLVMError("Failed to build branch conditional".to_string()))?;


        // build then block
        self.builder.position_at_end(then_bb);
        then_branch.accept(self)?;

        // get the final block that the builder ended up in (in the event of nested scopes)
        let final_then_bb = self.builder.get_insert_block()
            .ok_or_else(|| CompilerError::LLVMError("Failed to get most relevant insert block".to_string()))?;

            // Only emit a branch to continuation if the block doesn't already terminate (e.g., via a return)
            if final_then_bb.get_terminator().is_none() {
                self.builder.build_unconditional_branch(cont_bb)            
                    .map_err(|_| CompilerError::LLVMError("Failed to build branch conditional".to_string()))?;
            }


        // build else block (if it exists)
        if let Some(actual_else_bb) = else_bb {
            if let Some(actual_else_stmt) = else_branch {
                self.builder.position_at_end(actual_else_bb);
                actual_else_stmt.accept(self)?;

                // get the final block that the builder ended up in (in the event of nested scopes)
                let final_else_bb = self.builder.get_insert_block()
                    .ok_or_else(|| CompilerError::LLVMError("Failed to get most relevant insert block".to_string()))?;

                    if final_else_bb.get_terminator().is_none() {
                        self.builder.build_unconditional_branch(cont_bb)            
                            .map_err(|_| CompilerError::LLVMError("Failed to build branch conditional".to_string()))?;
                    }
            }
        }
        self.builder.position_at_end(cont_bb);

        Ok(())
    }

    fn visit_return_stmt(&mut self, keyword: &Token, value: &Expr) -> Result<(), CompilerError> {
        todo!()
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Statement) -> Result<(), CompilerError> {
        let parent = self.fn_value_opt.ok_or_else(|| {
            CompilerError::LLVMError("No active function context for while loop".to_string())
        })?;

        // initialize the three required branches
        let cond_bb = self.context.append_basic_block(parent, "while_cond");
        let body_bb = self.context.append_basic_block(parent, "while_body");
        let after_bb = self.context.append_basic_block(parent, "while_after");

        self.builder.build_unconditional_branch(cond_bb)
            .map_err(|_| CompilerError::LLVMError("Failed to branch to while conditional".to_string()))?;

        // build while condition block and evaluate conditional truthyness
        self.builder.position_at_end(cond_bb);
        let cond_expr = condition.accept(self)?;
        let cond_truthy = self.is_truthy(cond_expr)?;

        self.builder.build_conditional_branch(cond_truthy, body_bb, after_bb)
            .map_err(|_| CompilerError::LLVMError("Failed to build while conditional".to_string()))?;

        // build while body block
        self.builder.position_at_end(body_bb);
        body.accept(self)?;

        // get the final block that the builder ended up in (in the event of nested scopes)
        let final_body_bb = self.builder.get_insert_block()
            .ok_or_else(|| CompilerError::LLVMError("Failed to get most relevant insert block".to_string()))?;

        // Recursively jump back to the loop condition and execute the loop body
        if final_body_bb.get_terminator().is_none() {
            self.builder.build_unconditional_branch(cond_bb)            
                .map_err(|_| CompilerError::LLVMError("Failed to build branch conditional".to_string()))?;
        }

        self.builder.position_at_end(after_bb);

        Ok(())

    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<(), CompilerError> {

        let value = if matches!(initializer, Expr::Literal {value: Object::NULL, }) {
            self.literal_null_expr_node()?
        }
        else {
            initializer.accept(self)?
        };

        let basic_type = value.get_type();
        // Should store the variable into the context here
        let var_name = name.lexeme.as_str();

        let pointer = self.builder.build_alloca(basic_type, var_name)
            .map_err(|_| CompilerError::LLVMError("Stack Allocation failed".to_string()))?;

        self.builder.build_store(pointer, value)
            .map_err(|_| CompilerError::LLVMError("Failed to store initial value".to_string()))?;
        // and then stores the variable info in the environemnt here 
        self.environment.define(name, pointer, basic_type, true);

        Ok(())
    }
}

impl<'env, 'ctx> ExprVisitor<'ctx, Result<inkwell::values::BasicValueEnum<'ctx>, CompilerError>> for CodeGen<'env, 'ctx> {
    // Implement expression compilation methods here
    // Most expressions will return Ok(BasicValueEnum) representing the computed LLVM value
    

    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Box<Expr>>) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        todo!()
    }




    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        let parent = self.fn_value_opt.ok_or_else(|| CompilerError::LLVMError("No active function context".to_string()))?;

        let left_val = left.accept(self)?;
        let left_truthy = self.is_truthy(left_val)?;

        let right_bb = self.context.append_basic_block(parent, "logical_right");
        let merge_bb = self.context.append_basic_block(parent, "logical_merge");

        let initial_bb = self.builder.get_insert_block()
            .ok_or_else(|| CompilerError::LLVMError("Failed to get most relevant insert block".to_string()))?;


        if operator.token_type == TokenType::OR {
            // OR operation 
            // If lhs is truthy, bypass rhs eval and go straight to merge
            self.builder.build_conditional_branch(left_truthy, merge_bb, right_bb)
                .map_err(|_| CompilerError::LLVMError("Failed to build OR branch".to_string()))?;
        }
        else {
            // AND operation 
            // if lhs is falsy, bypass rhs and go straight to merge 
            self.builder.build_conditional_branch(left_truthy, right_bb, merge_bb)
                .map_err(|_| CompilerError::LLVMError("Failed to build AND branch".to_string()))?;
        }

        self.builder.position_at_end(right_bb);
        let right_val = right.accept(self)?;

        let final_right_bb = self.builder.get_insert_block()
            .ok_or_else(|| CompilerError::LLVMError("Failed to get most relevant RHS insert block".to_string()))?;

        if final_right_bb.get_terminator().is_none() {
            self.builder.build_unconditional_branch(merge_bb)            
                .map_err(|_| CompilerError::LLVMError("Failed to build merge branch".to_string()))?;
        }

        self.builder.position_at_end(merge_bb);

        let phi = self.builder.build_phi(left_val.get_type(), "logical_res")
                .map_err(|_| CompilerError::LLVMError("Failed to build PHI node".to_string()))?;

        phi.add_incoming(&[
            (&left_val, initial_bb),
            (&right_val, final_right_bb)
        ]);

        return Ok(phi.as_basic_value())

    }


    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<BasicValueEnum<'ctx>, CompilerError>{
        let assign_value = value.accept(self)?;
        // Should store the variable into the context here
        let basic_type = assign_value.get_type();
        let var_name = name.lexeme.as_str();

        let var_pointer = self.environment.lookup(name)?.pointer;

        self.builder.build_store(var_pointer, assign_value)
            .map_err(|_| CompilerError::LLVMError("Failed to store initial value".to_string()))?;

        self.environment.assign(name, var_pointer, basic_type , true)?;

        // Should assign return anything?
        return Ok(assign_value)
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

    fn is_truthy(&mut self, value:BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, CompilerError> {
        match value {
            BasicValueEnum::IntValue(int_val) => {
                let zero = int_val.get_type().const_int(0, false);
                // Returns true if int_val != 0
                let is_true = self.builder.build_int_compare(inkwell::IntPredicate::NE, int_val, zero, "int_truthy")
                    .map_err(|_| CompilerError::LLVMError("Failed to build int truthy comparison".to_string()))?;
                Ok(is_true)
            },

            BasicValueEnum::FloatValue(float_val) => {
                let zero = float_val.get_type().const_float(0.0);
                // Returns true if float_val != 0.0 (Ordered Not Equal)
                let is_true = self.builder.build_float_compare(inkwell::FloatPredicate::ONE, float_val, zero, "float_truthy")
                    .map_err(|_| CompilerError::LLVMError("Failed to build float truthy comparison".to_string()))?;
                Ok(is_true)
            },

            BasicValueEnum::PointerValue(ptr_val) => {
                // Create a null pointer constant of the exact same pointer type to compare against
                let null_ptr = ptr_val.get_type().const_null();

                // Returns true if ptr_val != null
                let is_true = self.builder.build_int_compare(inkwell::IntPredicate::NE, ptr_val, null_ptr, "ptr_truthy")
                    .map_err(|_| CompilerError::LLVMError("Failed to build pointer truthy comparison".to_string()))?;
                Ok(is_true)
            },

            _ => Err(CompilerError::LLVMError("This type cannot be evaluated for truthiness".to_string())),
        }    
    }
}


pub fn compile_program(program: &Vec<Statement>, file: &str) -> Result<(), CompilerError> {
    let context = Context::create();
    let module = context.create_module(file);
    let builder = context.create_builder();
    let mut environment = Environment::new();

    // The global context "main"
    let init_global_type = context.i32_type().fn_type(&[], false);
    let init_global_fn = module.add_function("__init_global", init_global_type, None);

    // entry point block for the LLVM program
    let entry_bb = context.append_basic_block(init_global_fn, "entry");
    builder.position_at_end(entry_bb);

    // Instantiate your code generator pass
    let mut codegen = CodeGen {
        context: &context,
        module,
        builder,
        environment: &mut environment,
        fn_value_opt: Some(init_global_fn),
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

