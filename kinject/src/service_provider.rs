use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};

pub static GLOBAL_SERVICE_PROVIDER: OnceLock<Mutex<ServiceProvider>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct ServiceProvider {
    services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl ServiceProvider {
    pub fn new() -> Self {
        ServiceProvider {
            services: HashMap::new(),
        }
    }

    pub fn resolve<T: 'static + Send + Sync>(&self) -> Arc<T> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|s| s.clone().downcast::<T>().ok())
            .expect("Service not found")
    }
    pub fn register<T, F>(&mut self, factory: F) -> &mut Self
    where
        T: 'static + Send + Sync,
        F: Fn(&ServiceProvider) -> T + 'static + Send + Sync,
    {
        let service = factory(self);
        self.services.insert(TypeId::of::<T>(), Arc::new(service));
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.services.clear();
        self
    }

    pub fn remove_service<T: 'static + Send + Sync>(&mut self) -> &mut Self {
        self.services.remove(&TypeId::of::<T>());
        self
    }

    pub fn build(&mut self) -> Self {
        if self.services.is_empty() {
            panic!("ServiceProvider is empty");
        }
        self.clone()
    }

    pub fn set_as_global(self) -> &'static Mutex<ServiceProvider> {
        if GLOBAL_SERVICE_PROVIDER.get().is_some() {
            panic!("Global ServiceProvider is already initialized");
        }
        GLOBAL_SERVICE_PROVIDER.set(Mutex::new(self)).unwrap();
        GLOBAL_SERVICE_PROVIDER.get().unwrap()
    }
    pub fn get_global() -> &'static Mutex<ServiceProvider> {
        GLOBAL_SERVICE_PROVIDER.get().unwrap()
    }

}
