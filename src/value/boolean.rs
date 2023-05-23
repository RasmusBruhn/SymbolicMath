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
}
