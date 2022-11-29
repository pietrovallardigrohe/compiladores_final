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

use std::io::BufRead;
use std::{fs, path::Path, io::Write};
use crate::lex::get_tokens;
use crate::syntax::get_syntax_errors;

fn main() {

    let mut out_file = fs::File::create("out.txt").expect("Could not create output file!");

    // std::env::set_var("RUST_BACKTRACE", "1");

    // Lê o caminho do arquivo
    println!("File Path");
    let mut file_path = String::new(); 
    std::io::stdin().lock().read_line(&mut file_path).expect("CANNOT READ INPUT");
    println!("\n\n");
    
    // Arquivo utilizado em testes
    // let file_path = String::from("src\\tests.txt"); 
    
    // Tenta ler o arquivo para a variável 'file'
    let Ok(raw_input) = fs::read_to_string(Path::new(&file_path.trim_end())) else {
        return println!("CANNOT OPEN FILE");
    };

    let mut input = raw_input.clone();
    input.push_str("$end$");

    let (tokens, errors) = get_tokens(&input);
    
    // Printa a análise lexica no arquivo de saida: "out.txt"
    writeln!(&mut out_file, "Lexical Analyzis==================").unwrap();
    writeln!(&mut out_file, "\nTOKENS------------------------").unwrap();
    tokens.iter().for_each(|_token| writeln!(&mut out_file, "{:?}", _token).unwrap());

    writeln!(&mut out_file, "\nLexical ERRORS------------------------").unwrap();
    errors.iter().for_each(|_token| writeln!(&mut out_file, "{:?}", _token).unwrap());

    // Printa a análise sintática no arquivo de saida
    writeln!(&mut out_file, "\nSyntactical Analyzis==================").unwrap();
    writeln!(&out_file, "Syntax ERRORS------------------------").unwrap();
    let syntax_errors = get_syntax_errors(tokens);
    syntax_errors.iter().for_each(|err| writeln!(&out_file, "Token: {:?} Message: {}", err.token, err.message).unwrap());
    
    writeln!(&out_file, "\nOutput: ").unwrap();
    let mut filtered_input = String::new();
    // Se for válido a análize printa o arquivo indicando onde estão os erros
    for (index, line) in raw_input.lines().enumerate() {
        if syntax_errors.iter().any(|err| err.token.line == index+1) {
            filtered_input.push_str(&format!("{line} <--- Syntax ERROR"));
        } else {
            filtered_input.push_str(&format!("{line}"));
        }
        filtered_input.push_str("\n");
    }
    writeln!(&out_file, "{}", filtered_input).unwrap();

    if syntax_errors.len() == 0 {
        writeln!(&out_file, "\nNO ERRORS FOUND").unwrap();
    }

    println!("DONE!");
}