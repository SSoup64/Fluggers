use super::node::Node;
use crate::gc::value::Value;

#[derive(Debug)]
pub struct FuncCall<'input> {
    func_name: &'input str,
    args: Vec<Node<'input>>
}

impl<'input> FuncCall<'input> {
    pub fn boxed(func_name: &'input str, args: Vec<Node<'input>>) -> Box<Self> {
        Box::new(Self { func_name, args })
    }

    pub fn evaluate(&self) -> Value {
        todo!();
    }
}
