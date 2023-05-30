use super;
use crate::traits;
use crate;

#[derive(Debug, PartialEq)]
pub struct Constant {
    value: bool,
}

impl Constant {
    pub fn new(value: bool) -> Self {
        Self {value}
    }

    pub fn to_bool(&self, database: crate::Database) -> Constant {
        Constant::new(self.value)
    }

    pub fn to_string(&self) -> String {
        (self.value as u32).to_string()
    }
}

#[derive(Debug, PartialEq)]
pub struct Reference {
    id: usize,
}

impl Reference {
    pub fn new(id: usize) -> Self {
        Self {id}
    }

    pub fn to_bool(&self, database: crate::Database) -> Constant {
        match database.get_variable(self.id) {
            Err(value) => Err(value),
            Ok(value) => Ok(value.to_bool(database))
        }
    }

    pub fn to_string(&self) -> String {
        (self.value as u32).to_string()
    }
}

#[derive(Debug, PartialEq)]
enum VariableType {
    Base,
    Value(Box<super::Boolean>),
    Ref(usize),
}

#[derive(Debug, PartialEq)]
#[traits::Variable]
pub struct Variable {
    name: String,
    id: Option<usize>,
    value: VariableType,
}

impl Variable {
    pub fn new(name: String, database: super::Database) -> Result<usize, super::DeclareError> {
        let variable = Variable {name, id: None, value: VariableType::Base};
        database.declare(variable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constant {
        use super::*;

        #[test]
        fn new() {
            let value_true = Constant::new(true);
            assert_eq!(value_true.value, true);

            let value_true = Constant::new(false);
            assert_eq!(value_true.value, false);
        }

        #[test]
        fn to_bool() {
            let value_true = Constant::new(true);
            assert_eq!(value_true.to_bool(), Constant {value: true});

            let value_false = Constant::new(false);
            assert_eq!(value_false.to_bool(), Constant {value: false});
        }

        #[test]
        fn to_string() {
            let value_true = Constant::new(true);
            assert_eq!(value_true.to_string(), "1");

            let value_false = Constant::new(false);
            assert_eq!(value_false.to_string(), "0");
        }
    }
}