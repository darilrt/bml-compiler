use crate::ir::ir::IR;

pub struct TranslatorHTML {
    pub output: String,
}

impl TranslatorHTML {
    pub fn translate(&mut self, ir: &IR) -> String {
        match ir {
            IR::Program(statements) => {
                for stmt in statements {
                    self.translate(stmt);
                }
            }
            IR::Element(tag, children) => {
                self.output.push_str(&format!("<{}>", tag));

                for child in children {
                    self.translate(child);
                }

                self.output.push_str(&format!("</{}>", tag));
            }
            IR::String(s) => {
                self.output.push_str(&s);
            }
            _ => {}
        }

        self.output.clone()
    }
}
