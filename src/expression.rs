#[allow(dead_code)]
pub trait Expression<T> {
    pub fn evaluate() -> T;
    pub fn compile() -> String;
    pub fn transpile() -> String;
}