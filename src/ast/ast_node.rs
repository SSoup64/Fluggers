use std::fmt::Debug;

pub trait AstNode: Debug {
    fn evaluate(&self);
}
