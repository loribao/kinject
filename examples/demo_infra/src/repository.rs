use demo_domain::irepository::IRepository;
use kinject::Injectable;

#[derive(Debug, Clone)]
pub struct Repository {}
impl Injectable for Repository {}

impl IRepository for Repository {
    fn new() -> Self {
        Repository {}
    }
    fn add(&self, left: i64, right: i64) -> i64 {
        left + right
    }
    fn sub(&self, left: i64, right: i64) -> i64 {
        left - right
    }
    fn mul(&self, left: i64, right: i64) -> i64 {
        left * right
    }
    fn div(&self, left: i64, right: i64) -> i64 {
        left / right
    }
}
