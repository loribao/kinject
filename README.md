# kinject

`kinject` é uma biblioteca de injeção de dependência para Rust, projetada para facilitar o gerenciamento de dependências em aplicações. Ele fornece um container de serviços chamado `ServiceProvider`, que permite registrar, resolver e gerenciar dependências de forma dinâmica e segura em ambientes multithread.

## Recursos Principais

- **Injeção de dependência**: Permite que objetos sejam criados e configurados automaticamente com suas dependências.
- **Container de serviços**: Um repositório central para registrar e resolver dependências.
- **Thread safety**: Utiliza estruturas como `Arc` e `Mutex` para garantir segurança em ambientes concorrentes.
- **Configuração global**: Permite configurar um container global acessível em toda a aplicação.

## Estrutura do Projeto

```
kinject/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── service_provider.rs
├── examples/
│   ├── demo_demo/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       └── setup.rs
│   ├── demo_domain/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── irepository.rs
│   │       ├── lib.rs
│   │       └── service.rs
│   └── demo_infra/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           └── repository.rs
```

## Como Usar

### 1. Configuração de Serviços

No arquivo `setup.rs`, registre os serviços no `ServiceProvider` de forma fluente e utilize `.set_as_global()` quantas vezes quiser para atualizar o provider global:

```rust
use demo_domain::service::{ServiceCalculator, Operator};
use demo_infra::repository::Repository;
use kinject::service_provider::ServiceProvider;

pub fn setup() {
    let mut service_provider = ServiceProvider::new();
    service_provider
        .register(|_| Repository::new())
        .register::<Arc<dyn IRepository>, _>(|_| Arc::new(Repository::new()))
        .register(|p| ServiceCalculator::new((*p.resolve::<Arc<dyn IRepository>>()).clone()))
        .set_as_global() // Torna global e permite continuar encadeando
        .register(|_| ServiceCalculator2::new())
        .set_as_global(); // Pode ser chamado novamente para atualizar o global
}
```

### 2. Uso no Código Principal

No arquivo `main.rs`, resolva os serviços registrados e use-os:

```rust
use demo_domain::service::{Operator, ServiceCalculator};
use kinject::service_provider::ServiceProvider;

fn main() {
    setup();

    let service_provider = ServiceProvider::get_global();
    let calculator = service_provider.resolve::<ServiceCalculator>();

    let result = calculator.calc(10, 5, Operator::Add);
    println!("Resultado: {}", result);
}
```

## Código do ServiceProvider

O `ServiceProvider` é o núcleo do `kinject`. Ele é responsável por gerenciar o ciclo de vida dos serviços registrados e permitir que dependências sejam resolvidas dinamicamente. Abaixo está o código atualizado do `ServiceProvider`:

```rust
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub static GLOBAL_SERVICE_PROVIDER: Mutex<Option<ServiceProvider>> = Mutex::new(None);

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
            .and_then(|s| s.clone().downcast::<Arc<T>>().ok())
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

    pub fn set_as_global(&mut self) -> &mut Self {
        let mut global = GLOBAL_SERVICE_PROVIDER.lock().unwrap();
        *global = Some(self.clone());
        self
    }

    pub fn get_global() -> ServiceProvider {
        let global = GLOBAL_SERVICE_PROVIDER.lock().unwrap();
        global.clone().expect("Global ServiceProvider not set")
    }
}
```

## Explicação do ServiceProvider

O `ServiceProvider` é um container de serviços que utiliza um `HashMap` para armazenar dependências. Cada serviço é identificado pelo seu `TypeId` e armazenado como um `Arc<dyn Any + Send + Sync>`, permitindo que múltiplas threads compartilhem o mesmo serviço de forma segura.

### Principais Mudanças

- **set_as_global fluente:** Agora retorna `&mut Self`, permitindo encadear registros e múltiplas chamadas.
- **Atualização do global:** O global pode ser sobrescrito quantas vezes quiser durante a execução.
- **Remoção do OnceLock:** O global agora é protegido por `Mutex<Option<ServiceProvider>>`, facilitando reinicialização e uso em testes.

### Segurança e Concorrência

- O uso de `Arc` e `Mutex` garante que o `ServiceProvider` seja seguro para uso em ambientes multithread.
- O método `set_as_global` pode ser chamado múltiplas vezes, sobrescrevendo o global.

### Benefícios

- **Flexibilidade:** Permite registrar e resolver serviços dinamicamente.
- **Reutilização:** Serviços podem ser compartilhados entre múltiplas threads.
- **Centralização:** Um ponto único para gerenciar dependências.
- **Reconfiguração:** Permite redefinir o global a qualquer momento.

### Limitações

- **Resolução de serviços não registrados:** Gera um `panic!` se o serviço não for encontrado.
- **Dependências circulares:** Pode causar loops infinitos durante a resolução.
- **Concorrência excessiva:** Pode causar contenção de threads devido ao uso de `Mutex`.

## Cenários de Uso

### Melhores Cenários de Uso

1. **Aplicações com muitas dependências**: Ideal para APIs, sistemas de backend e jogos.
2. **Ambientes multithread**: Compartilhamento seguro de serviços entre threads.
3. **Testes unitários**: Facilita a substituição de dependências reais por mocks.
4. **Configuração global de serviços**: Um ponto central de acesso às dependências.

### Cenários que Podem Dar Problemas

1. **Resolução de serviços não registrados**: Gera um `panic!` se o serviço não for encontrado.
2. **Concorrência excessiva**: Pode causar contenção de threads devido ao uso de `Mutex`.
3. **Dependências circulares**: Pode causar loops infinitos durante a resolução.
4. **Uso incorreto do container global**: Configurar o container global mais de uma vez gera um `panic!`.

## Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou enviar pull requests.

## Licença

Este projeto está licenciado sob a licença MIT. Veja o arquivo `LICENSE` para mais detalhes.