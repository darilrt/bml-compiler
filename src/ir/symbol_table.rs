use std::{collections::HashMap, rc::Rc};

pub struct Module {
    parent: Option<Box<Module>>,
    types: HashMap<String, Rc<Type>>,
    variables: HashMap<String, Rc<Type>>,
    modules: HashMap<String, Module>,
}

pub struct Type {
    fields: HashMap<String, String>,
}

impl Type {
    pub fn new() -> Type {
        Type {
            fields: HashMap::new(),
        }
    }
}

impl Module {
    pub fn new() -> Module {
        Module {
            parent: None,
            types: HashMap::new(),
            variables: HashMap::new(),
            modules: HashMap::new(),
        }
    }

    pub fn add_type(&mut self, name: &str, t: Type) {
        self.types.insert(name.to_string(), Rc::new(t));
    }

    pub fn add_variable(&mut self, name: &str, t: Type) {
        self.variables.insert(name.to_string(), Rc::new(t));
    }

    pub fn add_module(&mut self, name: &str, m: Module) {
        self.modules.insert(name.to_string(), m);
    }

    pub fn solve_type(&self, name: &str) -> Option<Rc<Type>> {
        if let Some(t) = self.types.get(name) {
            return Some(t.clone());
        }

        if let Some(p) = &self.parent {
            return p.solve_type(name);
        }

        None
    }

    pub fn solve_variable(&self, name: &str) -> Option<Rc<Type>> {
        if let Some(t) = self.variables.get(name) {
            return Some(t.clone());
        }

        if let Some(p) = &self.parent {
            return p.solve_variable(name);
        }

        None
    }

    pub fn solve_module(&self, name: &str) -> Option<&Module> {
        if let Some(t) = self.modules.get(name) {
            return Some(t);
        }

        if let Some(p) = &self.parent {
            return p.solve_module(name);
        }

        None
    }
}
