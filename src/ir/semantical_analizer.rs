use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use super::{
    ir::IR,
    symbol_table::{Module, Type},
};
use crate::parser::ast::*;

pub struct SemanticalAnalizer {
    module: Arc<Mutex<Module>>,
    current: Arc<Mutex<Module>>,
}

pub type AnalizeResult = (IR, Rc<Type>);

impl SemanticalAnalizer {
    pub fn new() -> SemanticalAnalizer {
        let module = Arc::new(Mutex::new(Module::new()));

        // Add types
        let mut m = module.lock().unwrap();
        m.add_type("none", Type::new());

        SemanticalAnalizer {
            module: module.clone(),
            current: module.clone(),
        }
    }

    pub fn none(&self) -> Rc<Type> {
        self.current.lock().unwrap().solve_type("none").unwrap()
    }

    pub fn analize(&self, ast: &Ast) -> AnalizeResult {
        match ast {
            Ast::Program(statements) => self.analize_program(statements),
        }
    }

    fn analize_program(&self, statements: &Vec<Stmt>) -> AnalizeResult {
        let mut ir = vec![];

        for stmt in statements {
            let (stmt_ir, _) = self.analize_stmt(stmt);
            ir.push(Box::new(stmt_ir));
        }

        let ir = IR::Program(ir);
        (ir, self.none())
    }

    pub fn analize_stmt(&self, stmt: &Stmt) -> AnalizeResult {
        match stmt {
            Stmt::DeclFunc(s) => self.analize_function_decl(s),
            _ => (IR::Empty, self.none()),
        }
    }

    pub fn analize_function_decl(&self, name: &String) -> AnalizeResult {
        // let mut m = self.module.lock().unwrap();
        // m.add_function(name.clone());

        (IR::DeclFunc(name.clone()), self.none())
    }
}
