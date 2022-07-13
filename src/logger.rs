use std::cell::Cell;

pub struct Logger {
    pub enabled: Cell<bool>,
}
#[allow(dead_code)]
impl Logger {
    pub fn info(&self, msg: String) {
        if self.enabled.get() {
            println!("[INFO] {}", msg);
        }
    }
    pub fn debug(&self, msg: String) {
        if self.enabled.get() {
            println!("[DEBUG] {}", msg);
        }
    }
    pub fn warn(&self, msg: String) {
        if self.enabled.get() {
            println!("[WARN] {}", msg);
        }
    }
    pub fn error(&self, msg: String) {
        if self.enabled.get() {
            panic!("[ERROR] {}", msg);
        }
    }
}
