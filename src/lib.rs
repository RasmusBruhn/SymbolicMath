pub mod value;
pub mod operator;

pub enum Expression {
//    Variable(Variable),
    Value(value::Value),
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
