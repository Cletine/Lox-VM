
#[derive (Debug, PartialEq, Clone)]
pub enum Object {
    STRING(String),
    NUMBER(f64),
    BOOL(bool),
    NULL,
}

impl Object {
    pub fn object_to_string(&self) -> String {
        match self {
            Object::STRING(string) => string.clone(),
            Object::NUMBER(number) => format!("{}", number),
            Object::NULL => "NULL".to_string(),
            Object::BOOL(boolean) =>  format!("{}", boolean),
        }
    }
}
