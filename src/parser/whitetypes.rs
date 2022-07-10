#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Type {
    Char,
    String,
    Integer,
    Float,
    Boolean,
    Null,
    ListChar,
    ListString,
    ListInteger,
    ListFloat,
    ListBoolean,
    Initialized,
    Void,
    Error,
}
impl Type {
    pub fn new(typ: &str) -> Type {
        let toks: Vec<&str> = typ.split('<').collect();
        match toks[0] {
            "char" => Type::Char,
            "string" => Type::String,
            "int" => Type::Integer,
            "float" => Type::Float,
            "bool" => Type::Boolean,
            "list" => match toks[1] {
                "char>" => Type::ListChar,
                "string>" => Type::ListString,
                "int>" => Type::ListInteger,
                "float>" => Type::ListFloat,
                "bool>" => Type::ListBoolean,
                _ => Type::Error,
            },
            "void" => Type::Void,
            _ => Type::Error,
        }
    }
    pub fn get_list_type(&self) -> Type {
        use Type::*;
        match self {
            Char => ListChar,
            String => ListString,
            Integer => ListInteger,
            Float => ListFloat,
            Boolean => ListBoolean,
            _ => Type::Error,
        }
    }
    pub fn get_type_from_list(&self) -> Type {
        use Type::*;
        match self {
            ListChar => Char,
            ListString => String,
            ListInteger => Integer,
            ListFloat => Float,
            ListBoolean => Boolean,
            _ => Type::Error,
        }
    }
    pub fn is_assignable_from(&self, other: Type) -> bool {
        if other == Type::Void {
            return false;
        } else if other == Type::Null {
            return true;
        } else if *self == other {
            return true;
        }
        false
    }
    pub fn is_list_type(&self) -> bool {
        use Type::*;
        match self {
            ListChar => true,
            ListString => true,
            ListInteger => true,
            ListBoolean => true,
            ListFloat => true,
            _ => false,
        }
    }
}
