#[derive(Debug)]
pub enum Expression {
    //Boolean(boolean::Boolean),
}

#[derive(Debug)]
struct Variable {
    name: String,
    value: Expression,
}

impl Variable {
    fn new(name: String, value: Expression) -> Self {
        Self {name, value}
    }
}

#[derive(Debug)]
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
            Err(NameExist)
        }
    }

    pub fn get_key(&mut self, name: &str) -> Option<usize> {
        self.dict.get(name)
    }

    pub fn get_expression(&self, id: usize) -> Option<&Expression> {
        match self.list.get(id) {
            None => None,
            Some(variable) => Some(variable.value),
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
            let value = Variable::new(String::from("x"), Expression {});
            assert_eq!(value.name, "x");
        }
    }

    mod database {
        use super::*;

        #[test]
        fn new() {
            let database = Database::new();
            assert_eq!(database.dict.len(), 0);
            assert_eq!(database.list.len(), 0);
        }

        #[test]
        fn declare() {
            let mut database = Database::new();
            
            if let Err(error) = database.declare("x") {
                panic!("An error occured while declaring x: {:?}", error);
            }

            assert_eq!(database.list.len(), 1);
            assert_eq!(database.dict.len(), 1);

            if let Err(error) = database.declare("y") {
                panic!("An error occured while declaring y: {:?}", error);
            }

            assert_eq!(database.list.len(), 2);
            assert_eq!(database.dict.len(), 2);

            if let Ok(_) = database.declare("x") {
                panic!("An error did not occur when declaring x twice");
            }

            assert_eq!(database.list.len(), 2);
            assert_eq!(database.dict.len(), 2);

            if let Err(error) = database.declare("z") {
                panic!("An error occured while declaring z: {:?}", error);
            }

            assert_eq!(database.list.len(), 3);
            assert_eq!(database.dict.len(), 3);
        }

        #[test]
        fn get_key() {
            let mut database = Database::new();
            
            if let Some(value) = database.get_key("x") {
                panic!("Got a key from database when none was added: {:?}", value);
            }

            if let Err(error) = database.declare("x") {
                panic!("An error occured while declaring x: {:?}", error);
            }

            if let None = database.get_key("x") {
                panic!("Unable to find x after x was added");
            }

            if let Some(value) = database.get_key("y") {
                panic!("Got y from database when x was added: {:?}", value);
            }

            if let Err(error) = database.declare("y") {
                panic!("An error occured while declaring y: {:?}", error);
            }

            if let None = database.get_key("x") {
                panic!("Unable to find x after y was added");
            }

            if let None = database.get_key("y") {
                panic!("Unable to find y after y was added");
            }

            if let Ok(_) = database.declare("y") {
                panic!("An error did not occured while declaring y twice");
            }

            if let None = database.get_key("x") {
                panic!("Unable to find x after y was added twice");
            }

            if let None = database.get_key("y") {
                panic!("Unable to find y after y was added twice");
            }
        }

        #[test]
        fn get_name() {
            let mut value = Database::new();

            if let Some(value) = database.get_name(0) {
                panic!("Got a name from database when none was added: {:?}", value);
            }

            if let Err(error) = database.declare("x") {
                panic!("An error occured while declaring x: {:?}", error);
            }

            match database.get_name(0) {
                None => panic!("Unable to find x after x was added");
                Some(value) => panic!("Looked for x but found {value} after x was added")
            }

            if let Some(value) = database.get_key("y") {
                panic!("Got y from database when x was added: {:?}", value);
            }

            if let Err(error) = database.declare("y") {
                panic!("An error occured while declaring y: {:?}", error);
            }

            if let None = database.get_key("x") {
                panic!("Unable to find x after y was added");
            }

            if let None = database.get_key("y") {
                panic!("Unable to find y after y was added");
            }

            if let Ok(_) = database.declare("y") {
                panic!("An error did not occured while declaring y twice");
            }

            if let None = database.get_key("x") {
                panic!("Unable to find x after y was added twice");
            }

            if let None = database.get_key("y") {
                panic!("Unable to find y after y was added twice");
            }
        }
    }
}
