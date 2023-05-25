//pub mod real;
pub mod boolean;

pub enum Value {
    Boolean(boolean::Boolean),
    //Integer,
    //Fraction,
    //Real(real::Real),
    //Imaginary,
    //Vector,
    //Matrix,
    //Tensor,
    //Set
}

use std::collections::HashMap;

pub struct Database {
    dict: HashMap<String, usize>,
    list: Vec<Variable>,
}

impl Database {
    pub fn new() -> Self {
        Self {dict: HashMap::new(), list: Vec::new()}
    }

    pub fn get_key(&mut self, name: &str) -> usize {
        match self.dict.get(name) {
            Some(value) => *value,
            None => {
                self.list.push(Variable::new(String::from(name)));
                self.dict.insert(String::from(name), self.list.len() - 1);
                self.list.len() - 1
            }
        }
    }

    pub fn get_variable(&self, id: usize) -> Option<&Variable> {
        self.list.get(id)
    }
}

pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Self {name: name}
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use super::*;

        #[test]
        fn new() {
            let mut value = Database::new();
            assert_eq!(value.dict.len(), 0);
            assert_eq!(value.list.len(), 0);

            value.dict.insert(String::from("a"), 1);
            value.list.push(Variable::new(String::from("x")));

            let value = Database::new();
            assert_eq!(value.dict.len(), 0);
            assert_eq!(value.list.len(), 0);
        }

        #[test]
        fn get_key() {
            let mut value = Database::new();
            
            assert_eq!(value.get_key("a"), 0);
            assert_eq!(value.get_key("a"), 0);
            assert_eq!(value.get_key("b"), 1);
            assert_eq!(value.get_key("a"), 0);
            assert_eq!(value.get_key("c"), 2);
            assert_eq!(value.get_key("b"), 1);
            assert_eq!(value.get_key("c"), 2);

            assert_eq!(value.dict.len(), 3);
            assert_eq!(value.list.len(), 3);
        }

        #[test]
        fn get_variable() {
            let mut value = Database::new();
            value.get_key("a");
            value.get_key("b");

            let variable = value.get_variable(0);
            if let None = variable {
                panic!("Variable must not be None")
            }

            let variable = value.get_variable(1);
            if let None = variable {
                panic!("Variable must not be None")
            }

            let variable = value.get_variable(2);
            if let Some(_) = variable {
                panic!("Variable must not be Some")
            }
        }
    }

    mod variable {
        use super::*;

        #[test]
        fn new() {
            let value = Variable::new(String::from("x"));
            assert_eq!(value.name, "x");
        }

        #[test]
        fn get_name() {
            let value = Variable::new(String::from("x"));
            assert_eq!(value.get_name(), "x");
        }
    }
}
