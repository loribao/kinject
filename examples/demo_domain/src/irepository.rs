use std::fmt::Debug;

use kinject::Injectable;


pub trait IRepository: Injectable {
    fn new() -> Self where Self: Sized;
    fn add(&self, left: i64, right: i64) -> i64;
    fn sub(&self, left: i64, right: i64) -> i64;
    fn mul(&self, left: i64, right: i64) -> i64;
    fn div(&self, left: i64, right: i64) -> i64;
}