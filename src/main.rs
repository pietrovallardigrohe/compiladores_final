/*
* Pietro Vallardi Grohe
* Compiladores
* Etapa 4
* 09/12/2022
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
    // Arquivo de saida, Cria ou apaga o conteúdo
    let Ok(mut out_file) = fs::File::create("out\\out.txt") else {
        return println!("Could not create output file!");
    };

    // Arquivo utilizado em testes
    let file_path = String::from("src\\tests.txt"); 

    // Lê o caminho do arquivo de entrada
    // Retire o comentário para ler um arquivo diferente
    
    // println!("File Path");
    // let mut file_path = String::new(); 
    // std::io::stdin().lock().read_line(&mut file_path).expect("CANNOT READ INPUT");
    // println!("\n\n");
    
    
    // Tenta ler o arquivo para a variável 'file'
    let Ok(raw_input) = fs::read_to_string(Path::new(&file_path.trim_end())) else {
        return println!("CANNOT OPEN FILE");
    };

    let mut input = raw_input.clone();
    input.push_str("EndOfInput");

    // Análise léxica
    let (tokens, lexical_errors) = get_tokens(&input);

    // Printa a análise léxica no arquivo de saida: "out.txt"

    let mut output_string = String::new();

    output_string.push_str("Lexical Analyzis==================\n");
    output_string.push_str("\nTOKENS------------------------\n");
    tokens.iter().for_each(|_token| output_string.push_str(&format!("{:?}\n", _token)));

    // Printa os errors léxicos
    output_string.push_str("\nLexical Errors------------------------\n");
    lexical_errors.iter().for_each(|_token| output_string.push_str(&format!("{:?}\n", _token)));

    // Análise Sintática
    let syntax_errors = syntax_test(&tokens);

    // Printa a análise sintática no arquivo de saida: "out.txt"
    output_string.push_str("\nSyntactical Analyzis==================\n");
    output_string.push_str("Syntax ERRORS------------------------\n");
    syntax_errors.iter().for_each(|err| output_string.push_str(&format!("Token:{:?} Message: {}\n", err.token, err.message)));
    
    output_string.push_str("\nOutput:\n");
    // Se for válido a análize printa o arquivo indicando onde estão os erros
    for (index, line) in raw_input.lines().enumerate() {
        output_string.push_str(line);
        for error in &syntax_errors {
            if error.token.line == index + 1 {
                output_string.push_str(&format!(" <--- Syntax Error || Message: {}", error.message));
                // continue 'line_iter;
            }
        }
        output_string.push('\n');
    }

    // Escreve as saídas no arquivo de saída
    writeln!(&mut out_file, "{output_string}").expect("CANNOT WRITE TO FILE");

    if syntax_errors.is_empty() {
        output_string.push_str("\nNO ERRORS FOUND");
    }

    println!("DONE!");
}