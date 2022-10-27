use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{Parser, error::Error};

#[derive(Parser)]
#[grammar = "Grammar/e1.pest"]
pub struct CompilatorParser;

fn main() -> Result<(), Error<Rule>> {

    let file = fs::read_to_string(r"src\target.txt").expect("unable to open file");
    
    // file.lines().filter(|e| !e.is_empty()).for_each(|s| print!("l: {}\n", s));
    for (index, line) in file.lines().enumerate().filter(|(_, e)| !e.is_empty()) {

        let parsed_line = CompilatorParser::parse(Rule::Token, line);
        match parsed_line {

            Ok(pairs) => {
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::Variable  
                        | Rule::Identifier
                        | Rule::Type 
                        | Rule::Char 
                        | Rule::Int 
                        | Rule::Float => println!("{} - {:?}: \"{}\"", index+1, pair.as_rule(), pair.as_str()),
                        _ => println!("Não reconhecido \"{}\" {:?} line: {}", pair.as_str(), pair.as_rule(), index)
                    }
                }
            },
            Err(error) => println!("Erro linha: {}, ---> \"{}\"", index+1, error.line().to_owned())

        }

    }

    // println!("{:?}", file.clone());
    // for pair in file.flatten() {
    //     match pair.as_rule() {
    //         // Rule::ddd => println!("DDD: {}", pair.as_str()),
    //         Rule::tokens => println!("Token: {}", pair.as_str()),
    //         Rule::variable => println!("Variável: {}", pair.as_str()),
    //         Rule::var_type => println!("Tipo: {}", pair.as_str()),
    //         Rule::identifier => println!("Identificador: {}", pair.as_str()),
    //         _ => println!("Não reconhecido {}", pair.as_str())
    //     }
    // }

    Ok(())

}