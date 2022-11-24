/*
* Pietro Vallardi Grohe
* Compiladores
* Etapa 2
* 24/11/2022
* 
* Dentro do cmd utilize dentro da pasta compiladores_final
* "cargo build" para compilar e "cargo run" para rodar o programa 
* Arquivo "tests.txt" contém diversas palavras que aceitam e rejeitam as regras de produção
* BNF Encontrada no arquivo Grammar/Lexer.pest e Grammar/Syntax.pest
*/
mod lex;
mod syntaxer;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::{fs, path::Path, io::Write};
use crate::lex::get_tokens;
use crate::syntaxer::syntax;

fn main() {
    
    let mut out_file = fs::File::create("out.txt").expect("Could not create output file!");

    // std::env::set_var("RUST_BACKTRACE", "1");
    //  Define o limite de chamadas das regras não terminais
    // pest::set_call_limit(NonZeroUsize::new(5000));
    println!();

    // // Lê o caminho do arquivo
    // // println!("File Path");
    // // let mut file_path = String::new(); 
    // // std::io::stdin().lock().read_line(&mut file_path).expect("CANNOT READ INPUT");
    // // println!("\n\n");
    
    // Arquivo utilizado em testes
    let file_path = String::from("src\\tests.txt"); 
    
    // Tenta ler o arquivo para a variável 'file'
    let Ok(mut raw_input) = fs::read_to_string(Path::new(&file_path.trim_end())) else {
        return println!("CANNOT OPEN FILE");
    };
    raw_input.push_str("\n$end$");

    // let mut filtered = String::new();

    let (tokens, errors) = get_tokens(&raw_input);
    
    writeln!(&mut out_file, "Lexical Analyzis==================").unwrap();
    writeln!(&mut out_file, "\nTOKENS------------------------").unwrap();
    tokens.iter().for_each(|_token| writeln!(&mut out_file, "{:?}", _token).unwrap());

    writeln!(&mut out_file, "\nERRORS------------------------").unwrap();
    errors.iter().for_each(|_token| writeln!(&mut out_file, "{:?}", _token).unwrap());

    syntax(tokens);

    // // Se for válido a análize printa o arquivo indicando onde estão os erros
    // match syntaxer::syntax(&lex_input) {
    //     Some(mut errors) => {
    //         errors.extend(lex_errors);
    //         errors.remove(&0);
    //         for (index, line) in raw_input.lines().enumerate() {
    //             if errors.contains(&(index+1)) {
    //                 filtered.push_str(&format!("ERROR-----> {line}"));
    //             } else {
    //                 filtered.push_str(&format!("{line}"));
    //             }
    //             filtered.push_str("\n");
    //         }
    //         println!("{}", filtered);
    //         if errors.len() == 0 {
    //             println!("No errors found");
    //         } 
    //         else {
    //             println!("Error at line {:?}", errors);
    //         }
    //     },
    //     None => {
    //         println!("ERROR INVALID BRACES");
    //     }
    // }
}