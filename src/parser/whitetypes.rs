
enum ListType {
    Char, String, Integer,
    Float, Boolean, Error
}

enum Type {
    Char, String, Integer,
    Float, Boolean, List
}
impl Type {
    fn get_list_type(typ: Type) -> ListType {
        match typ {
            Type::Char => ListType::Char,
            Type::String => ListType::String,
            Type::Integer => ListType::Integer,
            Type::Float => ListType::Float,
            Type::Boolean => ListType::Boolean,
            Type::List => ListType::Error
        }
    }
}