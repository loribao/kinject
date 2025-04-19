use kinject::service_provider::{ServiceProvider, GLOBAL_SERVICE_PROVIDER};
use std::sync::Arc;

use crate::{irepository::IRepository, service::Operator};

#[derive(Debug)]
pub struct ServiceCalculator2 {
    pub repository: Arc<dyn IRepository>,
}

impl ServiceCalculator2 {
    pub fn new() -> Self {
       
        let global = ServiceProvider::get_global();
        let _repository = global.resolve::<Arc<dyn IRepository>>();
        let repository = (*_repository).clone();
        ServiceCalculator2 { repository }
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
