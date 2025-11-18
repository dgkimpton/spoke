use crate::{
    asserts::Assert, body::Body, content::Content, equality::EqualityOperand, named::Named,
    operand::Operand, signature::Signature, suite::Suite,
};

pub enum CurrentRule {
    Suite(Suite),
    Body(Body),
    Signature(Signature),
    Named(Named),
    Content(Content),
    Assert(Assert),
    Operand(Operand),
    Eq(EqualityOperand),
}
