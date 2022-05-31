use std::sync::Arc;

dose::init!();

pub trait ATrait {
    fn a_string(&self) -> String;
}
pub type ATraitRef = Arc<dyn ATrait>;

pub struct AStruct {
    pub name: String,
}

impl ATrait for AStruct {
    fn a_string(&self) -> String {
        self.name.clone()
    }
}

pub struct Config;
