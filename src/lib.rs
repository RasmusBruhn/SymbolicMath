#[derive(Debug, PartialEq)]
pub enum Expression {
    Boolean,
    Integer,
}

#[derive(Debug, PartialEq)]
struct Variable {
    name: String,
    value: Expression,
}

impl Variable {
    fn new(name: String, value: Expression) -> Self {
        Self {name, value}
    }
}

#[derive(Debug, PartialEq)]
pub enum DeclareError {
    NameExist,
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    dict: HashMap<String, usize>,
    list: Vec<Variable>,
}

impl Database {
    pub fn new() -> Self {
        Self {dict: HashMap::new(), list: Vec::new()}
    }

    pub fn declare(&mut self, name: String, expression: Expression) -> Result<(), DeclareError> {
        if let None = self.dict.get(&name) {
            self.list.push(Variable::new(name.clone(), expression));
            self.dict.insert(name, self.list.len() - 1);
            Ok(())
        } else {
            Err(DeclareError::NameExist)
        }
    }

    pub fn get_key(&mut self, name: &str) -> Option<usize> {
        match self.dict.get(name) {
            Some(value) => Some(*value),
            None => None,
        }
    }

    pub fn get_expression(&self, id: usize) -> Option<&Expression> {
        match self.list.get(id) {
            None => None,
            Some(variable) => Some(&variable.value),
        }
    }

    pub fn get_name(&self, id: usize) -> Option<&str> {
        match self.list.get(id) {
            None => None,
            Some(variable) => Some(&variable.name),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    mod variable {
        use super::*;

        #[test]
        fn new() {
            let value = Variable::new(String::from("x"), Expression::Boolean);
            assert_eq!(value.name, "x");
        }
    }

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

        #[test]
        fn get_name() {
            let mut value = Database::new();

            if let Some(value) = value.get_name(0) {
                panic!("Got a name from database when none was added: {:?}", value);
            }

            if let Err(error) = value.declare("x".to_string(), Expression::Boolean) {
                panic!("An error occured while declaring x: {:?}", error);
            }

            match value.get_name(0) {
                None => panic!("Unable to find x after x was added"),
                Some(value) => assert_eq!(value, "x"),
            }

            if let Some(value) = value.get_name(1) {
                panic!("Got value 1 from database when x was added: {:?}", value);
            }

            if let Err(error) = value.declare("y".to_string(), Expression::Integer) {
                panic!("An error occured while declaring y: {:?}", error);
            }

            match value.get_name(0) {
                None => panic!("Unable to find x after y was added"),
                Some(value) => assert_eq!(value, "x"),
            }

            match value.get_name(1) {
                None => panic!("Unable to find y after y was added"),
                Some(value) => assert_eq!(value, "y"),
            }

            if let Some(value) = value.get_name(2) {
                panic!("Got value 2 from database when y was added: {:?}", value);
            }

            if let Ok(_) = value.declare("y".to_string(), Expression::Boolean) {
                panic!("An error did not occured while declaring y twice");
            }

            match value.get_name(0) {
                None => panic!("Unable to find x after y was added twice"),
                Some(value) => assert_eq!(value, "x"),
            }

            match value.get_name(1) {
                None => panic!("Unable to find y after y was added twice"),
                Some(value) => assert_eq!(value, "y"),
            }

            if let Some(value) = value.get_name(2) {
                panic!("Got value 2 from database when y was added twice: {:?}", value);
            }
        }
    }
}
