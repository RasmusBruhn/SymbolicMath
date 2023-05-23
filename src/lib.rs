pub mod constant;
pub mod operator;

pub enum Expression {
//    Variable(Variable),
    Constant(constant::Constant),
//    Operator(Operator)
}

//pub enum Variable {
//    Integer,
//    Fraction,
//    Real,
//    Imaginary,
//    Vector,
//    Matrix,
//    Tensor,
//    Set
//}


#[cfg(test)]
mod tests {

}
