pub struct Real {
    value: f64,
}

impl Real {
    pub fn new(value: f64) -> Self {
        Self {value: value}
    }

    pub fn to_float(&self) -> f64 {
        self.value
    }

    pub fn to_string(&self) -> String {
        format!("{:e}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let number = Real::new(0.);
        assert_eq!(number.value, 0.);

        let number = Real::new(-5.3);
        assert_eq!(number.value, -5.3);

        let number = Real::new(1.5e55);
        assert_eq!(number.value, 1.5e55);
    }

    #[test]
    fn to_float() {
        let number = Real::new(0.);
        assert_eq!(number.to_float(), 0.);

        let number = Real::new(-5.3);
        assert_eq!(number.to_float(), -5.3);

        let number = Real::new(1.5e55);
        assert_eq!(number.to_float(), 1.5e55);
    }

    #[test]
    fn to_string() {
        let number = Real::new(0.);
        assert_eq!(number.to_string(), "0e0");

        let number = Real::new(-5.3);
        assert_eq!(number.to_string(), "-5.3e0");

        let number = Real::new(1.5e55);
        assert_eq!(number.to_string(), "1.5e55");
    }
}
