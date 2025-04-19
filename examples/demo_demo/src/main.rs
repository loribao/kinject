mod setup;

use demo_domain::service::Operator;
use demo_domain::service::ServiceCalculator;
use kinject::service_provider::ServiceProvider;
use std::io;

fn main() {
    setup::setup();
    let container = ServiceProvider::get_global()
        .lock()
        .expect("Falha ao bloquear o ServiceProvider global");
    let service_calculator = container.resolve::<ServiceCalculator>();

    println!("Bem-vindo à calculadora!");
    println!("Digite a operação no formato: número1 operador número2");
    println!("Operadores disponíveis: +, -, *, /");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler entrada");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 3 {
        println!("Entrada inválida. Use o formato: número1 operador número2");
        return;
    }

    let num1 = parts[0].parse().expect("Número inválido");
    let operator = parts[1];
    let num2 = parts[2].parse().expect("Número inválido");

    let result = match operator {
        "+" => service_calculator.calc(num1, num2, Operator::Add),
        "-" => service_calculator.calc(num1, num2, Operator::Sub),
        "*" => service_calculator.calc(num1, num2, Operator::Mul),
        "/" => service_calculator.calc(num1, num2, Operator::Div),
        _ => {
            println!("Operador inválido. Use +, -, *, ou /.");
            return;
        }
    };

    println!("Resultado: {}", result);
}
