use std::sync::Arc;
use crate::irepository::IRepository;

#[derive(Debug,Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug)]
pub struct ServiceCalculator {
    pub repository: Arc<dyn IRepository>,
}


impl ServiceCalculator {
    pub fn new(repository: Arc<dyn IRepository>) -> Self {
        ServiceCalculator { repository }
    }
    pub fn calc(&self, left: i64, right: i64, oprator: Operator) -> i64 {
        match oprator {
            Operator::Add => self.repository.add(left, right),
            Operator::Sub => self.repository.sub(left, right),
            Operator::Mul => self.repository.mul(left, right),
            Operator::Div => self.repository.div(left, right),
        }
    }
}
