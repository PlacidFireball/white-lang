#[allow(dead_code)]
trait Expression<T> {
    fn evaluate() -> T;
    fn compile() -> String;
    fn transpile() -> String;
}