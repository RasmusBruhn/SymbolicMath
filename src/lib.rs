pub mod value;
pub mod operator;

pub enum Expression {
//    Variable(Variable),
    Value(value::Value),
//    Operator(Operator)
}

#[cfg(test)]
mod tests {

}
