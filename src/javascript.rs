
#[derive(Debug, Clone)]
pub struct JavaScript {
    src: String,
    level: String,
    // javascript runtime?
}
impl JavaScript {
    pub fn new() -> Self {
        Self {
            src: String::new(),
            level: String::new()
        }
    }

    pub fn get_src(&self) -> String {
        self.src.clone()
    }

    pub fn append(&mut self, value: String) -> &mut JavaScript {
        self.src.push_str(self.level.as_str());
        self.src.push_str(value.as_str());
        self
    }

    pub fn newline(&mut self) -> &mut JavaScript {
        self.src.push_str("\n");
        self
    }

    pub fn semicolon(&mut self) -> &mut JavaScript {
        self.src.push_str(";");
        self
    }

    pub fn indent(&mut self) -> &mut JavaScript {
        self.level.push_str("\t");
        self
    }

    pub fn outdent(&mut self) -> &mut JavaScript {
        self.level.pop();
        self
    }

    pub fn reset_level(&mut self) -> &mut JavaScript {
        self.level = String::new();
        self
    }
}

mod test {
    use super::*;
    // tests t o  d o
}