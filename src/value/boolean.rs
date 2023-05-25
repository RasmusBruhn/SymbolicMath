use super::Database;

pub enum Boolean {
    Constant(Constant),
    //Variable
}

pub struct Constant {
    value: bool,
}

impl Constant {
    pub fn new(value: bool) -> Self {
        Self {value: value}
    }

    pub fn export(&self) -> bool {
        self.value
    }

    pub fn to_string(&self) -> String {
        (self.value as i8).to_string()
    }
}

pub struct Variable {
    id: usize,
}

impl Variable {
    pub fn new(name: &str, database: &mut Database) -> Self {
        Self {id: database.get_key(name)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constant {
        use super::*;

        #[test]
        fn new() {
            let value = Constant::new(true);
            assert_eq!(value.value, true);

            let value = Constant::new(false);
            assert_eq!(value.value, false);
        }

        #[test]
        fn export() {
            let value = Constant::new(true);
            assert_eq!(value.export(), true);

            let value = Constant::new(false);
            assert_eq!(value.export(), false);
        }

        #[test]
        fn to_string() {
            let value = Constant::new(true);
            assert_eq!(value.to_string(), "1");

            let value = Constant::new(false);
            assert_eq!(value.to_string(), "0");
        }
    }

    mod variable {
        use super::*;

        #[test]
        fn new() {
            let mut database1 = Database::new();
            let mut database2 = Database::new();

            let x1 = Variable::new("x", &mut database1);
            assert_eq!(x1.id, 0);

            let y1 = Variable::new("y", &mut database1);
            assert_eq!(y1.id, 1);

            let x2 = Variable::new("x", &mut database1);
            assert_eq!(x2.id, 0);

            let y2 = Variable::new("y", &mut database2);
            assert_eq!(y2.id, 0);
        }
    }
}
