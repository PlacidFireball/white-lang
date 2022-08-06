pub struct Logger;
impl Logger {
    pub fn info(&self, msg: String) {
        println!("[INFO] {}", msg);
    }
    pub fn debug(&self, msg: String, _: bool) {
        if crate::DEBUG_INFO_LOGGING_ENABLED.with(|c| !c.get()) {
            return;
        }
        println!("[DEBUG] {}", msg);
    }
    pub fn warn(&self, msg: String) {
        println!("[WARN] {}", msg);
    }
    pub fn error(&self, msg: String) {
        panic!("[ERROR] {}", msg);
    }
}
