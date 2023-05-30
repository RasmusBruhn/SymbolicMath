pub mod value;

#[derive(Debug, PartialEq)]
pub enum Boolean {
    Constant(value::Constant),
    Reference(value::Reference),
    Variable(value::Variable),
}