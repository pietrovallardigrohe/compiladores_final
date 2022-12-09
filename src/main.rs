/*
* Pietro Vallardi Grohe
* Compiladores
* Etapa 3
* 25/11/2022
* 
* Dentro do cmd utilize dentro da pasta compiladores_final
* "cargo build" para compilar e "cargo run" para rodar o programa 
* Arquivo "tests.txt" contém entradas usadas para testes do desenvolvimento
*
* BNF Encontrada no arquivo RELATORIO.txt
*/
mod lex;
mod syntax;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[allow(unused_imports)]
use std::io::BufRead;
use std::{fs, path::Path, io::Write};

use crate::lex::get_tokens;
#[allow(unused_imports)]
use crate::syntax::{syntax_test};

fn main() {
    let mut out_file = fs::File::create("out.txt").expect("Could not create output file!");

    // std::env::set_var("RUST_BACKTRACE", "1");

    // Lê o caminho do arquivo
    // println!("File Path");
    // let mut file_path = String::new(); 
    // std::io::stdin().lock().read_line(&mut file_path).expect("CANNOT READ INPUT");
    // println!("\n\n");
    
    // Arquivo utilizado em testes
    let file_path = String::from("src\\tests.txt"); 
    
    // Tenta ler o arquivo para a variável 'file'
    let Ok(raw_input) = fs::read_to_string(Path::new(&file_path.trim_end())) else {
        return println!("CANNOT OPEN FILE");
    };

    let mut input = raw_input.clone();
    input.push_str("EndOfInput");

    // Análise léxica
    let (tokens, errors) = get_tokens(&input);

    // Printa a análise léxica no arquivo de saida: "out.txt"
    writeln!(&mut out_file, "Lexical Analyzis==================").unwrap();
    writeln!(&mut out_file, "\nTOKENS------------------------").unwrap();
    tokens.iter().for_each(|_token| writeln!(&mut out_file, "{:?}", _token).unwrap());

    // Printa os errors léxicos
    writeln!(&mut out_file, "\nLexical ERRORS------------------------").unwrap();
    errors.iter().for_each(|_token| writeln!(&mut out_file, "{:?}", _token).unwrap());

    // Análise Sintática
    let syntax_errors = syntax_test(&tokens);

    // Printa a análise sintática no arquivo de saida: "out.txt"
    writeln!(&mut out_file, "\nSyntactical Analyzis==================").unwrap();
    writeln!(&out_file, "Syntax ERRORS------------------------").unwrap();
    syntax_errors.iter().for_each(|err| writeln!(&out_file, "Token: {:?} Message: {}", err.token, err.message).unwrap());
    
    writeln!(&out_file, "\nOutput: ").unwrap();
    let mut filtered_input = String::new();
    // Se for válido a análize printa o arquivo indicando onde estão os erros
    for (index, line) in raw_input.lines().enumerate() {
        filtered_input.push_str(&format!("{line}"));
        for error in &syntax_errors {
            if error.token.line == index + 1 {
                filtered_input.push_str(&format!(" <--- Syntax Error || Message: {}", error.message));
                // continue 'line_iter;
            }
        }
        filtered_input.push_str("\n");
    }
    writeln!(&out_file, "{}", filtered_input).unwrap();

    if syntax_errors.len() == 0 {
        writeln!(&out_file, "\nNO ERRORS FOUND").unwrap();
    }

    println!("DONE!");
}