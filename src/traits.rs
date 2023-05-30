pub trait Variable {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_id(&self, id: usize) {
        if let Some(_) = self.id {
            panic!("Variable cannot get id twice");
        }

        self.id = id;
    }
}