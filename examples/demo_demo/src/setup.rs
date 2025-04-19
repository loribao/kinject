use demo_domain::{irepository::IRepository, service::ServiceCalculator};
use demo_infra::repository::Repository;
use kinject::service_provider::ServiceProvider;

//global

pub fn setup() {
    let mut service_provider = ServiceProvider::new();
    service_provider
        .register(|_| Repository::new())
        .register(|p| ServiceCalculator::new(p.resolve::<Repository>()))
        .build()
        .set_as_global();
}

#[cfg(test)]
mod tests {
    use demo_domain::service::Operator;

    use super::*;
    #[test]
    fn test_setup() {
        setup();
        let service_provider = ServiceProvider::get_global().lock().unwrap();
        let service_calculator = service_provider.resolve::<ServiceCalculator>();
        // Testa se o serviço foi adicionado corretamente

        assert_eq!(service_calculator.calc(1, 2, Operator::Add), 3);
        assert_eq!(service_calculator.calc(2, 1, Operator::Sub), 1);
        assert_eq!(service_calculator.calc(2, 2, Operator::Mul), 4);
        assert_eq!(service_calculator.calc(4, 2, Operator::Div), 2);
        let repository = service_provider.resolve::<Repository>();

        assert!(repository.add(1, 2) == 3);
        assert!(repository.sub(2, 1) == 1);
    }
}
