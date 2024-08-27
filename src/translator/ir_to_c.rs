use crate::ir::ir::IR;

pub struct TranslatorC {
    pub output: String,
}

impl TranslatorC {
    pub fn translate(&mut self, ir: &IR) -> String {
        match ir {
            IR::DeclFunc(s) => {
                let s = format!("void {}() {{\n}}\n", s);
                self.output.push_str(&s);
            }
            _ => {}
        }

        self.output.clone()
    }
}
