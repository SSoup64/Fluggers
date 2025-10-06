use crate::ast::func_decl::FuncDecl;

pub enum Value<'input> {
    Int(i128),
    Float(f64),
    Str(String),
    Func(&'input FuncDecl<'input>),
}
