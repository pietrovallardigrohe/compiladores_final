extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{Parser, iterators::{Pairs}, error::Error};
#[derive(Parser)]
#[grammar = "re.pest"]
pub struct Re;


fn main() {

    let test_string: &str = "(16) 28098500;(051)123456789; 1a;ABC-1234;CBA4321; 049 22337059; 049.223.370-59;   0.12 ; 12,20; 1.1; 0,0; <html> <a> </a> b </html>; a; abads; 12.345.678/0001-91;12.12;12345678901234;_as; a";

    for (index, value) in test_string.split(';').into_iter().enumerate() {

        let pairs: Result<Pairs<Rule>, Error<Rule>> = Re::parse(Rule::identifiers, value);

        if let Ok(p) = pairs {
            for pair in p.flatten() {
                match pair.as_rule() {
                    // Rule::ddd => println!("DDD: {}", pair.as_str()),
                    Rule::cellphone => println!("Celular: {}", pair.as_str()),
                    Rule::car_plate => println!("Placa de carro: {}", pair.as_str()),
                    Rule::cpf => println!("CPF: {}", pair.as_str()),
                    Rule::real_number => println!("Número Real: {}", pair.as_str()),
                    Rule::html_tag => println!("Tag HTML: {:?}", pair.as_str()),
                    Rule::portuguese_word => println!("Palavra: {}", pair.as_str()),
                    Rule::cnpj => println!("CNPJ: {}", pair.as_str()),
                    Rule::c_identifier => println!("Identificador C: {}", pair.as_str()),
                    _ => println!("Não reconhecido")
                }
            }
        } else {
            println!("ERROR PARSING VALUE \"{}\" AT {}", value, index);
        }

    }

}