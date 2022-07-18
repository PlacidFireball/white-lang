pub struct Logger;
impl Logger {
    pub fn info(&self, msg: String) {
        println!("[INFO] {}", msg);
    }
    pub fn debug(&self, msg: String) {
        println!("[DEBUG] {}", msg);
    }
    pub fn warn(&self, msg: String) {
        println!("[WARN] {}", msg);
    }
    pub fn error(&self, msg: String) {
        panic!("[ERROR] {}", msg);
    }
}
