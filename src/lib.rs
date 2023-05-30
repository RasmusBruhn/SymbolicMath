pub mod boolean;
pub mod traits;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Boolean,
    Integer,
}

#[derive(Debug, PartialEq)]
pub enum SymError {
    VariableExist("Variable already exists"),
    VariableMissing("Variable does not exist"),
    IDMissing("The variable ID does not exist"),
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    dict: HashMap<String, usize>,
    list: Vec<Expression: &impl traits::Variable>,
}

impl Database {
    pub fn new() -> Self {
        Self {dict: HashMap::new(), list: Vec::new()}
    }

    fn declare(&mut self, variable: &impl traits::Variable) -> Result<usize, SymError> {
        if let Some(_) = self.dict.get(variable.get_name()) {
            return Err(SymError::VariableExist)
        }

        let index = self.list.len();
        variable.set_id(index);
        self.dict.insert(variable.get_name().to_string(), index);
        self.list.push(variable);
        Ok(index)
    }

    pub fn get_key(&mut self, name: &str) -> Result<usize, SymError> {
        match self.dict.get(name) {
            Some(value) => Ok(*value),
            None => Err(VariableMissing),
        }
    }

    pub fn get_variable(&self, id: usize) -> Result<&Expression: &impl Variable, SymError> {
        match self.list.get(id) {
            Some(value) => Ok(value),
            None => Err(IDMissing),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use super::*;

        #[test]
        fn new() {
            let value = Database::new();
            assert_eq!(value.dict.len(), 0);
            assert_eq!(value.list.len(), 0);
        }

        #[test]
        fn declare() {
            let mut value = Database::new();
            
            let variable = Expression::Boolean;
            assert_eq!(match value.declare("x") {
                Err(error) => panic!("Unable to declare x: {:?}", error),
                Ok(f) => f(variable),
            }, 0);

            assert_eq!(value.dict.len(), 1);
            assert_eq!(value.list.len(), 1);

            let variable = Expression::Boolean;
            assert_eq!(match value.declare("y") {
                Err(error) => panic!("Unable to declare y: {:?}", error),
                Ok(f) => f(variable),
            }, 1);

            assert_eq!(value.dict.len(), 2);
            assert_eq!(value.list.len(), 2);

            let variable = Expression::Boolean;
            if let Ok(_) = value.declare("x") {
                panic!("Did not get error when declaring x twice");
            }

            assert_eq!(value.dict.len(), 2);
            assert_eq!(value.list.len(), 2);

            let variable = Expression::Boolean;
            assert_eq!(match value.declare("z") {
                Err(error) => panic!("Unable to declare z: {:?}", error),
                Ok(f) => f(variable),
            }, 2);

            assert_eq!(value.dict.len(), 3);
            assert_eq!(value.list.len(), 3);
        }

        #[test]
        #[should_panic]
        fn declare_panic() {
            let mut value = Database::new();
            
            let variable = Expression::Boolean;
            match value.declare("x") {
                Err(error) => panic!("Unable to declare x: {:?}", error),
                Ok(f) => 0,
            };

            match value.declare("y") {
                Err(error) => panic!("Unable to declare y: {:?}", error),
                Ok(f) => f(variable),
            };

        }
/*
        #[test]
        fn declare() {
            let mut value = Database::new();
            
            if let Err(error) = value.declare("x".to_string(), Expression::Boolean) {
                panic!("An error occured while declaring x: {:?}", error);
            }

            assert_eq!(value.list.len(), 1);
            assert_eq!(value.dict.len(), 1);

            if let Err(error) = value.declare("y".to_string(), Expression::Boolean) {
                panic!("An error occured while declaring y: {:?}", error);
            }

            assert_eq!(value.list.len(), 2);
            assert_eq!(value.dict.len(), 2);

            if let Ok(_) = value.declare("x".to_string(), Expression::Boolean) {
                panic!("An error did not occur when declaring x twice");
            }

            assert_eq!(value.list.len(), 2);
            assert_eq!(value.dict.len(), 2);

            if let Err(error) = value.declare("z".to_string(), Expression::Boolean) {
                panic!("An error occured while declaring z: {:?}", error);
            }

            assert_eq!(value.list.len(), 3);
            assert_eq!(value.dict.len(), 3);
        }
*//*
        #[test]
        fn get_key() {
            let mut value = Database::new();
            
            if let Some(value) = value.get_key("x") {
                panic!("Got a key from database when none was added: {:?}", value);
            }

            if let Err(error) = value.declare("x".to_string(), Expression::Boolean) {
                panic!("An error occured while declaring x: {:?}", error);
            }

            if let None = value.get_key("x") {
                panic!("Unable to find x after x was added");
            }

            if let Some(value) = value.get_key("y") {
                panic!("Got y from database when x was added: {:?}", value);
            }

            if let Err(error) = value.declare("y".to_string(), Expression::Boolean) {
                panic!("An error occured while declaring y: {:?}", error);
            }

            if let None = value.get_key("x") {
                panic!("Unable to find x after y was added");
            }

            if let None = value.get_key("y") {
                panic!("Unable to find y after y was added");
            }

            if let Ok(_) = value.declare("y".to_string(), Expression::Boolean) {
                panic!("An error did not occured while declaring y twice");
            }

            if let None = value.get_key("x") {
                panic!("Unable to find x after y was added twice");
            }

            if let None = value.get_key("y") {
                panic!("Unable to find y after y was added twice");
            }
        }
*//*
        #[test]
        fn get_expression() {
            let mut value = Database::new();

            if let Some(value) = value.get_expression(0) {
                panic!("Got a name from database when none was added: {:?}", value);
            }

            if let Err(error) = value.declare("x".to_string(), Expression::Boolean) {
                panic!("An error occured while declaring x: {:?}", error);
            }

            match value.get_expression(0) {
                None => panic!("Unable to find x after x was added"),
                Some(value) => assert_eq!(*value, Expression::Boolean),
            }

            if let Some(value) = value.get_expression(1) {
                panic!("Got value 1 from database when x was added: {:?}", value);
            }

            if let Err(error) = value.declare("y".to_string(), Expression::Integer) {
                panic!("An error occured while declaring y: {:?}", error);
            }

            match value.get_expression(0) {
                None => panic!("Unable to find x after y was added"),
                Some(value) => assert_eq!(*value, Expression::Boolean),
            }

            match value.get_expression(1) {
                None => panic!("Unable to find y after y was added"),
                Some(value) => assert_eq!(*value, Expression::Integer),
            }

            if let Some(value) = value.get_expression(2) {
                panic!("Got value 2 from database when y was added: {:?}", value);
            }

            if let Ok(_) = value.declare("y".to_string(), Expression::Boolean) {
                panic!("An error did not occured while declaring y twice");
            }

            match value.get_expression(0) {
                None => panic!("Unable to find x after y was added twice"),
                Some(value) => assert_eq!(*value, Expression::Boolean),
            }

            match value.get_expression(1) {
                None => panic!("Unable to find y after y was added twice"),
                Some(value) => assert_eq!(*value, Expression::Integer),
            }

            if let Some(value) = value.get_expression(2) {
                panic!("Got value 2 from database when y was added twice: {:?}", value);
            }
        }
*/
    }
}
