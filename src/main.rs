/*
 * Pietro Vallardi Grohe
 * Compiladores
 * Etapa 1 
 * 31/10/22
 * 
 * Dentro do cmd utilize dentro da pasta compiladores_final
 * "cargo build" e "cargo run" para compilar e rodar o programa 
 * Arquivo "tests.txt" contém diversas palavras que aceitam e rejeitam as regras de produção
 * BNF Encontrada no arquivo Grammar/etapa1.pest
 */
use std::{fs, num::NonZeroUsize, path::Path};
use pest::{Parser, iterators::Pairs, error::Error};

extern crate pest;
#[macro_use]
extern crate pest_derive;

// Macro para gerar o Parser e o Enum com as regras automáticamente a partir de um arquivo .pest
#[derive(Parser)] 
#[grammar = "Grammar/etapa1.pest"] 
pub struct CompilatorParser;    
//  Struct que conterá apenas a função parser

fn main() {
    //  Define o limite de chamadas das regras não terminais
    pest::set_call_limit(NonZeroUsize::new(5000));
    
    print!("\n\n");

    // Lê o caminho do arquivo
    println!("File Path");
    let mut file_path = String::new(); 
    std::io::stdin().read_line(&mut file_path).expect("CANNOT READ INPUT");

    // Tenta ler o arquivo para a variável 'file'
    if let Ok(file) = fs::read_to_string(Path::new(&file_path.trim_end())) {

        println!("\n---------------------\n");
        // Separa o arquivo em linhas
        for (line_count, raw_token) in file.lines().enumerate() {  
            /*
            * Faz o parse da linha e retorna um Resultado.
            * Caso aceite a gramática retorna os pares reconhecidos e se rejeita retorna um erro
            */
            let parsed_line: Result<Pairs<Rule>, Error<Rule>> = CompilatorParser::parse(Rule::Token, raw_token);
            match parsed_line {
                Ok(pairs) => {
                    for pair in pairs {
                        match pair.as_rule() {
                            Rule::Variable  
                            | Rule::Identifier
                            | Rule::Type 
                            | Rule::Char 
                            | Rule::Int 
                            | Rule::Float => println!("Line: {} = {:?}: \"{}\"", line_count+1, pair.as_rule(), pair.as_str()),
                            _ => println!("Não reconhecido \"{}\" Rule: {:?} Line: {}", pair.as_str(), pair.as_rule(), line_count+1)
                        }
                    }
                },
                Err(error) => {
                    println!("ERROR: Line: {}, ---> \"{}\"", line_count+1, error.line().to_owned());
                }
            }
        }
        print!("\n---------------------\n");

    } else {
        println!("\nCANNOT OPEN FILE");
    }

}