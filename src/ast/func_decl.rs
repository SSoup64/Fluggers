use super::expr_list::ExprList;

#[derive(Debug)]
pub struct FuncDecl<'input> {
    params: Vec<&'input str>,
    exprs: ExprList<'input>
}

impl<'input> FuncDecl<'input> {
    pub fn boxed(params: Vec<&'input str>, exprs: ExprList<'input>) -> Box<Self> {
        Box::new(Self { params, exprs })
    }

    pub fn evaluate(&self) {}
}

