use std::collections::HashMap;

use swc_atoms::JsWord;
use swc_ecma_ast::*;
use swc_ecma_visit::{noop_fold_type, Fold, FoldWith};

#[derive(Clone)]
struct MangleFold {}

pub fn mangle() -> impl Fold {
    MangleFold {}
}

impl Fold for MangleFold {
    noop_fold_type!();

    fn fold_module(&mut self, m: Module) -> Module {
        m.fold_children_with(&mut MangleDecl {
            used_names: &mut HashMap::new(),
        })
    }
}

pub struct MangleDecl<'a> {
    pub used_names: &'a mut HashMap<JsWord, String>,
}

impl<'a> Fold for MangleDecl<'a> {
    noop_fold_type!();

    fn fold_function(&mut self, m: Function) -> Function {
        m.fold_children_with(&mut MangleDecl {
            used_names: &mut HashMap::new(),
        })
    }

    fn fold_var_decl(&mut self, decl: VarDecl) -> VarDecl {
        decl.fold_children_with(&mut MangleIdentCollector {
            used_names: &mut self.used_names,
        })
    }

    fn fold_ident(&mut self, ident: Ident) -> Ident {
        if self.used_names.contains_key(&ident.sym) {
            return Ident {
                sym: self.used_names.get(&ident.sym).unwrap().clone().into(),
                ..ident
            };
        }

        ident
    }
}

pub struct MangleIdentCollector<'a> {
    pub used_names: &'a mut HashMap<JsWord, String>,
}

impl<'a> Fold for MangleIdentCollector<'a> {
    noop_fold_type!();

    fn fold_ident(&mut self, ident: Ident) -> Ident {
        if self.used_names.contains_key(&ident.sym) {
            return Ident {
                sym: self.used_names.get(&ident.sym).unwrap().clone().into(),
                ..ident
            };
        }

        let new_name = format!("_{}", self.used_names.len());
        self.used_names.insert(ident.sym.clone(), new_name.clone());

        Ident {
            sym: new_name.into(),
            ..ident
        }
    }
}
