#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Char,
    String,
    Integer,
    Float,
    Boolean,
    Null,
    Object,
    ListChar,
    ListString,
    ListInteger,
    ListFloat,
    ListBoolean,
    ListObject,
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
            Null => ListObject,
            Object => ListObject,
            ListChar => ListChar,
            ListString => ListString,
            ListInteger => ListInteger,
            ListFloat => ListFloat,
            ListBoolean => ListBoolean,
            ListObject => ListObject,
            Initialized => Error,
            Void => Error,
            Error => Error,
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
            ListObject => Object,
            _ => *self,
        }
    }
    pub fn is_assignable_to(&self, other: Type) -> bool {
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
            ListObject => true,
            _ => false,
        }
    }
}
