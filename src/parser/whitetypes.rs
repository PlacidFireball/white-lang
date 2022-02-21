pub(crate) enum ListType {
    Char,
    String,
    Integer,
    Float,
    Boolean,
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Type {
    Char,
    String,
    Integer,
    Float,
    Boolean,
    List,
    Null,
    Initialized,
    Void,
    Error,
}
impl Type {
    pub fn new(typ: &str) -> Type {
        match typ {
            "char" => Type::Char,
            "string" => Type::String,
            "int" => Type::Integer,
            "float" => Type::Float,
            "bool" => Type::Boolean,
            "list" => Type::Error,
            _ => Type::Error,
        }
    }
    pub fn get_list_type(&self) -> ListType {
        match self {
            Type::Char => ListType::Char,
            Type::String => ListType::String,
            Type::Integer => ListType::Integer,
            Type::Float => ListType::Float,
            Type::Boolean => ListType::Boolean,
            Type::List => ListType::Error,
            _ => ListType::Error,
        }
    }
}
