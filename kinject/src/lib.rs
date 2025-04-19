use std::fmt::Debug;

pub mod service_provider;
pub trait Injectable: Send + Sync + Debug{}
