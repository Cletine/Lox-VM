use std::collections::HashMap; use inkwell::values::PointerValue;
use inkwell::types::BasicTypeEnum;
use crate::lox::Token;
use crate::CompilerError;




#[derive(Clone, Debug)]
pub struct VariableInfo<'ctx> {
    pub pointer: PointerValue<'ctx>,
    pub ty: BasicTypeEnum<'ctx>,
    pub is_mutable: bool,
}

pub struct Environment <'ctx> {
    scope: Vec<HashMap<&'ctx str, VariableInfo<'ctx>>>,
}

impl<'ctx> Environment <'ctx> {
    
    pub fn new() -> Self {
        Self {
            scope: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) -> Result<(), CompilerError> {
        self.scope.push(HashMap::new());
        return Ok(())
    }

    pub fn exit_scope(&mut self) -> Result<(), CompilerError> {
        if self.scope.len() > 1 {
            self.scope.pop();
            return Ok(())
        }
        else {
            return Err(CompilerError::VariableEnvironment("Internal Compiler Error: Mismatched scope lifecycle tracking.".to_string()));
        }
    }


    pub fn define(&mut self, name: &'ctx Token, pointer: PointerValue<'ctx>, ty: BasicTypeEnum<'ctx>, is_mutable: bool) -> Result<(), CompilerError>{
        let var_name = name.lexeme.as_str();
        if let Some(current_scope) = self.scope.last_mut() {
            let info = VariableInfo {pointer: pointer, ty:ty, is_mutable:is_mutable};
            current_scope.insert(var_name, info);
            return Ok(())
        }
        else {
            return Err(CompilerError::VariableEnvironment("Internal Compiler Error: Unable to Define Variable in the Current Scope.".to_string()));
        }
    }


    pub fn lookup(&mut self, name: &'ctx Token) -> Result<&VariableInfo<'ctx>, CompilerError> {
        let var_name = name.lexeme.as_str();
        for current_scope in self.scope.iter().rev() {
            if let Some(info) = current_scope.get(var_name) {
                return Ok(info)
            }
        }
        return Err(CompilerError::UndefinedVariable(format!("Variable '{}' not found.", name.lexeme)))
    }

    pub fn assign(&mut self, name: &'ctx Token, pointer: PointerValue<'ctx>, ty: BasicTypeEnum<'ctx>, is_mutable: bool) -> Result<(), CompilerError> {
        let info = VariableInfo {pointer: pointer, ty:ty, is_mutable:is_mutable};
        let var_name = name.lexeme.as_str();
        for current_scope in self.scope.iter_mut().rev() {
            if current_scope.contains_key(var_name) {
                current_scope.insert(var_name, info);
                return Ok(())
            }
        }
        return Err(CompilerError::UndefinedVariable(format!("Internal Compiler Error: Undefined Variable '{}'", name.lexeme)));
    }
}


