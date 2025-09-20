use super::expr_list::ExprList;

#[derive(Debug)]
pub struct Func<'input> {
    exprs: ExprList<'input>
}

impl<'input> Func<'input> {
    pub fn boxed(exprs: ExprList<'input>) -> Box<Self> {
        Box::new(Self { exprs })
    }

    pub fn evaluate(&self) {}
}

