use std::collections::HashMap;
use inkwell::values::PointerValue;
use inkwell::types::BasicTypeEnum;



#[derive(Clone, Debug)]
pub struct VariableInfo<'ctx> {
    pub pointer: PointerValue<'ctx>,
    pub ty: BasicTypeEnum<'ctx>,
    pub is_mutable: bool,
}

pub struct Environment <'ctx> {

    scope: Vec<Hashmap<String, Object<'ctx>>>,
}

impl<'ctx> Environmnet <'ctx> {
    
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
        else {
            panic!("Internal Compiler Error: Mismatched scope lifecycle tracking.");
        }
    }

    pub fn define(&mut self, name: String, pointer: PointerValue<'ctx>, ty: BasicTypeEnum, is_mutable: bool) {
        if let Some(current_scope) = self.scopes.last_mut() {
            let info = VariableInfo {pointer: pointer, ty:ty, is_mutable:is_mutable};
            current_scope.insert(name, info);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&VariableInfo<'ctx>> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    pub fn assign(&self, name: Token, pointer: PointerValue<'ctx>, ty: BasicTypeEnum, is_mutable: bool) {
        let info = VariableInfo {pointer: pointer, ty:ty, is_mutable:is_mutable};
        for current_scope in self.scopes.iter().rev() {
            if current_scope.contains_key(name) {
                return current_scope.insert(name, info);
            }
        }
        panic!("Internal Compiler Error: Undefined Variable '{}'", name.lexeme);
    }
}


